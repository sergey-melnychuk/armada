use std::{net::SocketAddr, time::Duration};

use armada::{
    arg::Args,
    cfg::{Config, Profile},
    ctx::{Context, Shared},
    db::Storage,
    eth::EthClient,
    seq::SeqClient,
    sync::{self, Event, Source},
    util::U64,
};
use yakvdb::typed::DB;

const SECOND: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args: Args = armada::arg::resolve()?;
    let token = &args.infura_token;
    let is_metrics_reporting_enabled = args.flags.contains("metrics");

    let mainnet = Profile {
        network: "mainnet".to_string(),
        eth_url: format!("https://mainnet.infura.io/v3/{token}"),
        seq_url: "https://alpha-mainnet.starknet.io".to_string(),
        eth_contract_address: "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4".to_string(),
    };

    let testnet = Profile {
        network: "testnet".to_string(),
        eth_url: format!("https://goerli.infura.io/v3/{token}"),
        seq_url: "https://alpha4.starknet.io".to_string(),
        eth_contract_address: "0xde29d060D45901Fb19ED6C6e959EB22d8626708e".to_string(),
    };

    let integration = Profile {
        network: "integration".to_string(),
        eth_url: format!("https://goerli.infura.io/v3/{token}"),
        seq_url: "https://external.integration.starknet.io".to_string(),
        eth_contract_address: "0xd5c325D183C592C94998000C5e0EED9e6655c020".to_string(),
    };

    let profile = match args.network {
        name if name == "mainnet" => mainnet,
        name if name == "testnet" => testnet,
        name if name == "integration" => integration,
        name => {
            anyhow::bail!(
                "Unsupported network: {}. Supported networks: mainnet, testnet, integration.",
                name
            );
        }
    };

    let storage_path = &format!("{}/{}", args.data_dir, profile.network);
    tracing::info!(
        network = profile.network,
        storage = storage_path,
        "Armada is starting..."
    );

    let rpc_bind_addr = "0.0.0.0:9000";
    let eth_poll_delay = 120 * SECOND;
    let seq_poll_delay = 30 * SECOND;

    let config = Config::new(
        profile.network.clone(),
        rpc_bind_addr.parse()?,
        SECOND,
        seq_poll_delay,
        eth_poll_delay,
        profile.eth_contract_address.to_string(),
    );

    let eth = EthClient::new(&profile.eth_url);
    let seq = SeqClient::new(&profile.seq_url);
    let db = Storage::new(storage_path).await;
    let shared = Shared::default();

    let ctx = Context::new(eth, seq, shared, db, config);
    let ctx = if is_metrics_reporting_enabled {
        let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
        let handle = builder
            .add_global_label("app", "armada")
            .add_global_label("network", &profile.network)
            .install_recorder()
            .expect("failed to install prometheus recorder");
        ctx.with_metrics(handle)
    } else {
        ctx
    };
    let source = Source::new(ctx.clone());
    source.add("uptime", sync::poll_uptime, SECOND).await;
    source.add("gateway", sync::poll_seq, seq_poll_delay).await;
    source.add("ethereum", sync::poll_eth, eth_poll_delay).await;
    let tx = source.tx();
    let syncer = armada::sync::sync(source, sync::handler).await;

    let range = {
        let idx = ctx.db.blocks_index.read().await;
        let min = idx.min()?.map(|val| val.into_u64());
        let max = idx.max()?.map(|val| val.into_u64());
        min.zip(max)
    };
    if let Some((lo, hi)) = range {
        {
            let sync = &mut ctx.shared.lock().await.sync;
            sync.lo = Some(lo);
            sync.hi = Some(hi);
        }
        if lo > 0 {
            use armada::db::Repo;
            let key = U64::from_u64(lo);
            let lo_block_hash = ctx.db.blocks_index.read().await.lookup(&key)?.unwrap();
            let lo_block = ctx.db.blocks.get(&lo_block_hash.into_str()).await?.unwrap();
            let lo_parent_hash = lo_block.block_header.parent_hash.0;
            tx.send(Event::PullBlock(lo - 1, lo_parent_hash)).await.ok();
        }
        tracing::info!(synced=?(lo, hi), "Sync running");
    } else {
        tracing::info!("Sync running");
    }

    {
        let ctx = ctx.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let missing = armada::util::detect_gaps(ctx).await?;
            if !missing.is_empty() {
                tracing::info!(total = missing.len(), "Sync gap detected");
            }
            let zero = armada::api::gen::Felt::try_new("0x0")?;
            for number in missing {
                let event = Event::PullBlock(number, zero.clone());
                tx.send(event).await?;
                tokio::time::sleep(SECOND).await;
            }
            Ok::<(), anyhow::Error>(())
        });
    }

    {
        let ctx = ctx.clone();
        let tx = tx.clone();
        let len = 2000;
        tokio::spawn(async move {
            if let Some((number, hash)) = armada::util::check_chain(ctx, len).await? {
                tracing::info!(at = number, "Broken chain detected");
                let event = Event::PullBlock(number, armada::api::gen::Felt::try_new(&hash)?);
                tx.send(event).await?;
            } else {
                tracing::info!(length = len, "Chain head validated successfully");
            }
            Ok::<(), anyhow::Error>(())
        });
    }

    let addr: SocketAddr = rpc_bind_addr.parse()?;
    let (addr, server) = armada::rpc::serve(&addr, ctx).await;
    tracing::info!(at=?addr, "RPC server listening");

    syncer.done().await;
    server.done().await;
    Ok(())
}

use std::{net::SocketAddr, time::Duration};

use armada::{
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

    let home = std::env::var("HOME")?;
    let token = std::env::var("INFURA_TOKEN")?;

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

    let profile = match std::env::args().nth(1).as_ref() {
        Some(name) if name == "mainnet" => mainnet,
        Some(name) if name == "testnet" => testnet,
        Some(name) => {
            anyhow::bail!(
                "Unsupported network: {}. Supported networks: mainnet, testnet.",
                name
            );
        }
        None => {
            anyhow::bail!("Network is not defined. Supported networks: mainnet, testnet.");
        }
    };

    tracing::info!(network = profile.network, "Armada is starting...");

    let storage_path = &format!("{home}/Temp/armada/{}", profile.network);

    let rpc_bind_addr = "0.0.0.0:9000";
    let eth_poll_delay = 120 * SECOND;
    let seq_poll_delay = 30 * SECOND;

    let config = Config::new(
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
            let zero = armada::api::gen::Felt::try_new("0x0")?;
            for number in missing {
                let event = Event::PullBlock(number, zero.clone());
                tx.send(event).await?;
                tokio::time::sleep(SECOND).await;
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

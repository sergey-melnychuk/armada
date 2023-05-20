use std::{net::SocketAddr, time::Duration};

use armada::{
    cfg::Config,
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
    let eth_url = &format!("https://mainnet.infura.io/v3/{token}");

    let seq_url = "https://alpha-mainnet.starknet.io";

    // let eth_contract_address = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
    let eth_contract_address = "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4";

    let storage_path = &format!("{home}/Temp/armada/mainnet");

    let rpc_bind_addr = "0.0.0.0:9000";
    let eth_poll_delay = 120 * SECOND;
    let seq_poll_delay = 30 * SECOND;
    let config = Config::new(SECOND, eth_contract_address.to_string());

    let eth = EthClient::new(eth_url);
    let seq = SeqClient::new(seq_url);
    let db = Storage::new(storage_path).await;
    let shared = Shared::default();

    let ctx = Context::new(eth, seq, shared, db, config);
    let source = Source::new(ctx.clone());
    source.add("uptime", sync::poll_uptime, SECOND).await;
    source.add("seq", sync::poll_seq, seq_poll_delay).await;
    source.add("eth", sync::poll_eth, eth_poll_delay).await;
    let tx = source.tx();
    let syncer = armada::sync::sync(source, sync::handler).await;

    let (lo, hi) = {
        let idx = ctx.db.blocks_index.read().await;
        let min = idx.min()?.unwrap_or_default().into_u64();
        let max = idx.max()?.unwrap_or_default().into_u64();
        (min, max)
    };
    if lo > 0 {
        use armada::db::Repo;
        let key = U64::from_u64(lo);
        let lo_block_hash = ctx.db.blocks_index.read().await.lookup(&key)?.unwrap();
        let lo_block = ctx.db.blocks.get(&lo_block_hash.into_str()).await?.unwrap();
        let lo_parent_hash = lo_block.block_header.parent_hash.0;
        tx.send(Event::PullBlock(lo_parent_hash)).await.ok();
    }
    tracing::info!(synced=?(lo, hi), "Sync running");

    let done = is_done::is_done(ctx.db.blocks_index.clone());

    let addr: SocketAddr = rpc_bind_addr.parse()?;
    let (addr, server) = armada::rpc::serve(&addr, ctx).await;
    tracing::info!(at=?addr, "RPC server listening");

    done.await;
    syncer.stop();
    server.stop();

    syncer.done().await;
    server.done().await;

    tracing::warn!("Armada is out :micdrop:");
    Ok(())
}

pub mod is_done {
    use std::{sync::Arc, time::Duration};

    use armada::util::{U256, U64};
    use tokio::sync::RwLock;
    use yakvdb::typed::{Store, DB};

    pub async fn is_done(index: Arc<RwLock<Store<U64, U256>>>) {
        let delay = 5 * Duration::from_secs(60);
        tokio::spawn(async move {
            loop {
                if let Some(min) = index.read().await.min()? {
                    if min.into_u64() == 0 {
                        tracing::info!("Sync is complete");
                        break;
                    }
                }
                tokio::time::sleep(delay).await;
            }
            Ok::<_, anyhow::Error>(())
        })
        .await
        .map(|_| ())
        .ok();
    }
}

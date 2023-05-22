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
    let eth_url = &format!("https://goerli.infura.io/v3/{token}");

    let seq_url = "https://alpha4.starknet.io";

    let eth_contract_address = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

    let storage_path = &format!("{home}/Temp/armada/data");
    let rpc_bind_addr = "0.0.0.0:9000";
    let eth_poll_delay = 30 * SECOND;
    let seq_poll_delay = 120 * SECOND;
    let config = Config::new(SECOND, eth_contract_address.to_string());

    let eth = EthClient::new(eth_url);
    let seq = SeqClient::new(seq_url);
    let db = Storage::new(storage_path);
    let shared = Shared::default();

    let (lo, hi) = {
        let idx = db.blocks_index.read().await;
        let min = idx.min()?.unwrap_or_default().into_u64();
        let max = idx.max()?.unwrap_or_default().into_u64();
        (min, max)
    };
    tracing::info!(lo, hi, "Sycned blocks");

    let ctx = Context::new(eth, seq, shared, db, config);
    let source = Source::new(ctx.clone());
    source.add("uptime", sync::poll_uptime, SECOND).await;
    source.add("seq", sync::poll_seq, seq_poll_delay).await;
    source.add("eth", sync::poll_eth, eth_poll_delay).await;
    let tx = source.tx();
    let syncer = armada::sync::sync(source, sync::handler).await;

    if lo > 0 {
        use armada::db::Repo;
        let key = U64::from_u64(lo);
        let lo_block_hash = ctx.db.blocks_index.read().await.lookup(&key)?.unwrap();
        let lo_block = ctx.db.blocks.get(&lo_block_hash.into_str())?.unwrap();
        let lo_parent_hash = lo_block.block_header.parent_hash.0;
        tx.send(Event::PullBlock(lo_parent_hash)).await.ok();
    }

    let addr: SocketAddr = rpc_bind_addr.parse()?;
    let (addr, server) = armada::rpc::serve(&addr, ctx).await;
    tracing::info!(at=?addr, "RPC server listening");
    server.done().await;
    syncer.done().await;

    Ok(())
}

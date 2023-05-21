use std::{net::SocketAddr, time::Duration};

use armada::{
    cfg::Config,
    ctx::{Context, Shared},
    db::Storage,
    eth::EthClient,
    seq::SeqClient,
    sync::{self, Source},
};
use yakvdb::typed::DB;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = std::env::var("INFURA_TOKEN")?;
    let eth_url = &format!("https://goerli.infura.io/v3/{token}");

    let seq_url = "https://alpha4.starknet.io";

    let eth_contract_address = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

    let storage_path = "./target/db";
    let rpc_bind_addr = "0.0.0.0:9000";
    let poll_delay = Duration::from_secs(1);
    let config = Config::new(poll_delay, eth_contract_address.to_string());

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
    // TODO: Start syncing down from min block

    let ctx = Context::new(eth, seq, shared, db, config);
    let source = Source::new(ctx.clone());
    source.add("uptime", sync::poll_uptime, poll_delay).await;
    source.add("seq", sync::poll_seq, poll_delay * 10).await;
    source.add("eth", sync::poll_eth, poll_delay * 10).await;
    let syncer = armada::sync::sync(source, sync::handler).await;

    let addr: SocketAddr = rpc_bind_addr.parse()?;
    let (addr, server) = armada::rpc::serve(&addr, ctx).await;
    tracing::info!(at=?addr, "RPC server listening");
    server.wait().await;
    syncer.wait().await;

    Ok(())
}

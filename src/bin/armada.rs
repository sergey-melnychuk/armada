use std::{net::SocketAddr, time::Duration};

use armada::{
    cfg::Config,
    ctx::{Context, Shared},
    db::Storage,
    eth::EthClient,
    seq::SeqClient,
    sync::{self, Source},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let eth_url = "http://localhost:3000/eth";
    let seq_url = "http://localhost:3000/seq";
    let storage_path = "./target/db";
    let rpc_bind_addr = "0.0.0.0:9000";
    let poll_delay = Duration::from_secs(1);
    let eth_contract_address = "0x0".to_string();
    let config = Config::new(poll_delay, eth_contract_address);

    let eth = EthClient::new(eth_url);
    let seq = SeqClient::new(seq_url);
    let db = Storage::new(storage_path);
    let shared = Shared::default();

    let ctx = Context::new(eth, seq, shared, db, config);
    let source = Source::new(ctx.clone());
    source.add("uptime", sync::poll_uptime, poll_delay).await;
    let syncer = armada::sync::sync(source, sync::handler).await;

    let addr: SocketAddr = rpc_bind_addr.parse()?;
    let (_, server) = armada::rpc::serve(&addr, ctx).await;
    server.wait().await;
    syncer.wait().await;

    Ok(())
}

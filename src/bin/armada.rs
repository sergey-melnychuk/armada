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
async fn main() {
    let eth = EthClient::new("http://localhost:3000/eth");
    let seq = SeqClient::new("http://localhost:3000/seq");
    let shared = Shared::default();
    let db = Storage::new("./target/db");
    let config = Config::new(Duration::from_secs(1));

    let ctx = Context::new(eth, seq, shared, db, config);
    let source = Source::new(ctx.clone());
    source
        .add("uptime", sync::poll_uptime, Duration::from_secs(1))
        .await;
    let syncer = armada::sync::sync(source, sync::handler).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    let (_, server) = armada::rpc::serve(&addr, ctx).await;
    server.wait().await;
    syncer.wait().await;
}

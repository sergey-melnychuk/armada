use std::net::SocketAddr;

use armada::{
    ctx::{Context, Shared},
    db::Storage,
    eth::EthClient,
    seq::SeqClient,
};

#[tokio::main]
async fn main() {
    let eth = EthClient::new("http://localhost:3000/eth");
    let seq = SeqClient::new("http://localhost:3000/seq");
    let shared = Shared::default();
    let db = Storage::new("./target/db");

    let ctx = Context::new(eth, seq, shared, db);
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    let server = armada::rpc::serve(&addr, ctx).await;
    server.wait().await
}

use std::net::SocketAddr;

use armada::{ctx::Context, db::Storage};

#[tokio::main]
async fn main() {
    let ctx = Context::new(Storage {});
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    let server = armada::rpc::serve(&addr, ctx).await;
    server.wait().await
}

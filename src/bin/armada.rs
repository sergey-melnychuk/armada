use std::net::SocketAddr;

use armada::ctx::Context;

#[tokio::main]
async fn main() {
    let ctx = Context {};
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    armada::rpc::serve(addr, ctx).await
}

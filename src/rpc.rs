use std::net::SocketAddr;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use iamgroot::jsonrpc;
use serde::{Deserialize, Serialize};

use crate::{api::gen, ctx::Context, eth::EthApi, seq::SeqApi, util::Waiter};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Request {
    Single(jsonrpc::Request),
    Batch(Vec<jsonrpc::Request>),
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Response {
    Single(jsonrpc::Response),
    Batch(Vec<jsonrpc::Response>),
}

async fn handle_request<ETH, SEQ>(
    State(state): State<Context<ETH, SEQ>>,
    Json(req): Json<Request>,
) -> impl IntoResponse
where
    ETH: EthApi + Send + Sync + 'static,
    SEQ: SeqApi + Send + Sync + 'static,
{
    match req {
        Request::Single(req) => {
            log::info!("method: {}", req.method);
            let res = gen::handle(&state, &req);
            Json(Response::Single(res))
        }
        Request::Batch(req) => {
            let res = req
                .into_iter()
                .map(|req| {
                    log::info!("method: {}", req.method);
                    gen::handle(&state, &req)
                })
                .collect::<Vec<_>>();
            Json(Response::Batch(res))
        }
    }
}

pub async fn serve<ETH, SEQ>(addr: &SocketAddr, ctx: Context<ETH, SEQ>) -> (SocketAddr, Waiter)
where
    ETH: EthApi + Send + Sync + Clone + 'static,
    SEQ: SeqApi + Send + Sync + Clone + 'static,
{
    let app = Router::new()
        .route("/rpc/v0.3", post(handle_request))
        .with_state(ctx);

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    let server = axum::Server::bind(addr).serve(app.into_make_service());

    let addr = server.local_addr();

    // https://docs.rs/hyper/latest/hyper/server/struct.Server.html#method.with_graceful_shutdown
    let graceful = server.with_graceful_shutdown(async {
        rx.await.ok();
    });

    let jh = tokio::spawn(async move { graceful.await.unwrap() });

    (addr, Waiter::new(jh, tx))
}

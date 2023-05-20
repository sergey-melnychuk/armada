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

struct RpcError(anyhow::Error);

impl IntoResponse for RpcError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("RPC error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for RpcError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn handle_request<ETH, SEQ>(
    State(state): State<Context<ETH, SEQ>>,
    Json(req): Json<Request>,
) -> Result<impl IntoResponse, RpcError>
where
    ETH: EthApi + Clone + Send + Sync + 'static,
    SEQ: SeqApi + Clone + Send + Sync + 'static,
{
    match req {
        Request::Single(req) => {
            log::info!("method: {}", req.method);
            let res = tokio::task::spawn_blocking(move || gen::handle(&state, &req))
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
            Ok(Json(Response::Single(res)))
        }
        Request::Batch(reqs) => {
            let mut ret = Vec::with_capacity(reqs.len());
            for req in reqs {
                log::info!("method: {}", req.method);
                let state = state.clone();
                let res = tokio::task::spawn_blocking(move || gen::handle(&state, &req))
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
                ret.push(res)
            }
            Ok(Json(Response::Batch(ret)))
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

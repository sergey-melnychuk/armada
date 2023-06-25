use std::net::SocketAddr;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use iamgroot::jsonrpc;
use reqwest::StatusCode;
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
    ETH: EthApi,
    SEQ: SeqApi,
{
    match req {
        Request::Single(req) => {
            log::info!("method: {}", req.method);
            metrics::counter!("rpc_request", 1, "method" => req.method.clone());
            let res = gen::handle(&state, &req).await;
            Ok(Json(Response::Single(res)))
        }
        Request::Batch(reqs) => {
            let mut ret = Vec::with_capacity(reqs.len());
            for req in reqs {
                log::info!("method: {}", req.method);
                metrics::counter!("rpc_request", 1, "method" => req.method.clone());
                let state = state.clone();
                let res = gen::handle(&state, &req).await;
                ret.push(res)
            }
            Ok(Json(Response::Batch(ret)))
        }
    }
}

async fn handle_metrics<ETH, SEQ>(
    State(state): State<Context<ETH, SEQ>>,
) -> Result<impl IntoResponse, RpcError>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    if let Some(handle) = state.metrics.as_ref() {
        return Ok(handle.render().into_response());
    }
    Ok((StatusCode::NOT_FOUND, "Not Found").into_response())
}

async fn handle_status<ETH, SEQ>(
    State(state): State<Context<ETH, SEQ>>,
) -> Result<impl IntoResponse, RpcError>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    let (lo, hi) = {
        let sync = &mut state.shared.lock().await.sync;
        (sync.lo, sync.hi)
    };

    let ratio = lo
        .zip(hi)
        .map(|(lo, hi)| (lo as f64, hi as f64))
        .map(|(lo, hi)| (hi - lo) / hi * 100.0)
        .map(|r| format!("{r:.2}%"))
        .unwrap_or("".to_string());

    Ok(Html(format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>Sync Status</title></head><body><h1>{}..{}</h1><h1>{}</h1></body></html>"#,
        lo.map(|lo| format!("{lo}")).unwrap_or("?".to_string()),
        hi.map(|hi| format!("{hi}")).unwrap_or("?".to_string()),
        ratio,
    )))
}

pub async fn serve<ETH, SEQ>(addr: &SocketAddr, ctx: Context<ETH, SEQ>) -> (SocketAddr, Waiter)
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    let app = Router::new()
        .route("/rpc/v0.3", post(handle_request))
        .route("/metrics", get(handle_metrics))
        .route("/sync/status", get(handle_status))
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

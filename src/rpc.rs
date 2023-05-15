use std::net::SocketAddr;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use iamgroot::jsonrpc;
use serde::{Deserialize, Serialize};

use crate::ctx::Context;

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

async fn handle_request(
    State(state): State<Context>,
    Json(req): Json<Request>,
) -> impl IntoResponse {
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

pub async fn serve(addr: SocketAddr, ctx: Context) {
    let app = Router::new()
        .route("/rpc/v0.3", post(handle_request))
        .with_state(ctx);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

use std::{cell::RefCell, net::SocketAddr};

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use iamgroot::jsonrpc;
use serde::{Deserialize, Serialize};
use tokio::{sync::oneshot::Sender, task::JoinHandle};

use crate::{api::gen, ctx::Context, eth::EthApi, seq::SeqApi};

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

pub struct Server {
    jh: RefCell<Option<JoinHandle<()>>>,
    tx: RefCell<Option<Sender<()>>>,
    addr: SocketAddr,
}

impl Server {
    fn new(jh: JoinHandle<()>, tx: Sender<()>, addr: SocketAddr) -> Self {
        Self {
            jh: RefCell::new(Some(jh)),
            tx: RefCell::new(Some(tx)),
            addr,
        }
    }

    pub fn unfold(&mut self) -> Option<(JoinHandle<()>, Sender<()>)> {
        if self.jh.borrow().is_none() || self.tx.borrow().is_none() {
            return None;
        }
        let jh = self.jh.borrow_mut().take().unwrap();
        let tx = self.tx.borrow_mut().take().unwrap();
        Some((jh, tx))
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub async fn wait(&self) {
        if self.jh.borrow().is_none() {
            return;
        }
        let jh = self.jh.borrow_mut().take().unwrap();
        if let Err(e) = jh.await {
            log::error!("Server task terminated with error: {e}");
        }
    }

    pub async fn stop(&self) {
        if self.tx.borrow().is_none() {
            return;
        }
        let tx = self.tx.borrow_mut().take().unwrap();
        if tx.send(()).is_err() {
            log::error!("Server shutdown attempt failed");
        }
    }
}

pub async fn serve<ETH, SEQ>(addr: &SocketAddr, ctx: Context<ETH, SEQ>) -> Server
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

    Server::new(jh, tx, addr)
}

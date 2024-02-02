use std::time::Duration;

use armada::{
    cfg::Config,
    ctx::{Context, Shared},
    db::Storage,
    util::Waiter,
};
use iamgroot::jsonrpc;
use serde::{de::DeserializeOwned, Serialize};
use tempdir::TempDir;

use self::eth::TestEth;
use self::seq::TestSeq;

pub mod eth;
pub mod seq;

pub struct Test {
    dir: Option<TempDir>,
    url: String,
    http: reqwest::Client,
    server: Waiter,
    pub ctx: Context<TestEth, TestSeq>,
}

impl Test {
    pub async fn new() -> Self {
        let id = format!("{}", uuid::Uuid::new_v4());
        let dir = TempDir::new(&id).expect("temp-dir");

        let eth = TestEth::new();
        let seq = TestSeq::new();

        let shared = Shared::default();
        let db = Storage::new(dir.path()).await;

        let config = Config::new(
            "test".to_string(),
            ([127, 0, 0, 1], 0).into(),
            Duration::from_secs(1),
            Duration::from_secs(1),
            Duration::from_secs(1),
            "0x0".to_string(),
        );

        let ctx = Context::new(eth, seq, shared, db, config);

        let http = reqwest::ClientBuilder::new().build().expect("http");
        let (addr, server) =
            armada::rpc::serve(&([127, 0, 0, 1], 0).into(), ctx.clone()).await;
        let url = format!("http://{}/rpc/v0.3", addr);

        Self {
            dir: Some(dir),
            url,
            http,
            server,
            ctx,
        }
    }

    #[allow(dead_code)] // IDK why but clippy thinks this method is a dead code (it is not)
    pub async fn rpc<T: Serialize, R: DeserializeOwned>(
        &self,
        req: T,
    ) -> anyhow::Result<R> {
        let mut res: jsonrpc::Response = self
            .http
            .post(&self.url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        if res.result.is_none() {
            if let Some(error) = res.error {
                anyhow::bail!("JSON-RPC error: {error:?}");
            } else {
                anyhow::bail!("JSON-RPC error: result missing");
            }
        }

        let res: R = serde_json::from_value(res.result.take().unwrap())?;
        Ok(res)
    }
}

// More on async in Drop impl: https://stackoverflow.com/a/75584109
impl Drop for Test {
    fn drop(&mut self) {
        if let Some(dir) = self.dir.take() {
            dir.close().ok();
        }
        if let Some((jh, tx)) = self.server.unfold() {
            tokio::spawn(async move {
                tx.send(()).ok();
                jh.await.ok();
            });
        }
    }
}

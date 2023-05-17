use armada::{ctx::{Context, Shared}, db::Storage, rpc::Server};
use iamgroot::jsonrpc;
use serde::{de::DeserializeOwned, Serialize};
use tempdir::TempDir;

pub mod eth;
pub mod seq;

pub struct Test {
    dir: Option<TempDir>,
    url: String,
    db: Storage,
    http: reqwest::Client,
    server: Server,
}

impl Test {
    pub async fn new() -> Self {
        let id = format!("{}", uuid::Uuid::new_v4());
        let dir = TempDir::new(&id).expect("temp-dir");

        let eth = eth::TestEth {};
        let seq = seq::TestSeq {};

        let shared = Shared::default();
        let db = Storage::new(dir.path());
        let ctx = Context::new(eth, seq, shared, db.clone());

        let http = reqwest::ClientBuilder::new().build().expect("http");
        let server = armada::rpc::serve(&([127, 0, 0, 1], 0).into(), ctx).await;
        let url = format!("http://{}/rpc/v0.3", server.addr());
        Self {
            dir: Some(dir),
            url,
            db,
            http,
            server,
        }
    }

    pub fn db(&self) -> &Storage {
        &self.db
    }

    pub fn db_mut(&mut self) -> &mut Storage {
        &mut self.db
    }

    pub async fn rpc<T: Serialize, R: DeserializeOwned>(&self, req: T) -> anyhow::Result<R> {
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

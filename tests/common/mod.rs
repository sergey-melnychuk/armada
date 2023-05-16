use armada::{ctx::Context, rpc::Server};
use iamgroot::jsonrpc;
use serde::{de::DeserializeOwned, Serialize};

pub struct Test {
    url: String,
    http: reqwest::Client,
    server: Server,
}

impl Test {
    pub async fn new(ctx: Context) -> Self {
        let http = reqwest::ClientBuilder::new().build().expect("http");
        let server = armada::rpc::serve(&([127, 0, 0, 1], 0).into(), ctx).await;
        let url = format!("http://{}/rpc/v0.3", server.addr());
        Self { url, http, server }
    }

    pub async fn call<T: Serialize, R: DeserializeOwned>(&self, req: T) -> anyhow::Result<R> {
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
        if let Some((jh, tx)) = self.server.unfold() {
            tokio::spawn(async move {
                tx.send(()).ok();
                jh.await.ok();
            });
        }
    }
}

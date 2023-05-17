use std::{sync::Arc, time::Duration};

use futures::Future;
use tokio::sync::{mpsc, Mutex, Notify};

pub struct Source<T, C> {
    tx: mpsc::Sender<T>,
    rx: mpsc::Receiver<T>,
    go: Arc<Notify>,
    ctx: Arc<Mutex<C>>,
}

impl<T: Send + 'static, C: Send + 'static> Source<T, C> {
    pub fn new(ctx: C) -> Self {
        let (tx, rx) = mpsc::channel(32);
        let go = Arc::new(Notify::new());
        let ctx = Arc::new(Mutex::new(ctx));
        Self { tx, rx, go, ctx }
    }

    pub async fn add<F, G>(&self, name: &str, f: F, poll: Duration)
    where
        F: (Fn(Arc<Mutex<C>>) -> G) + Send + 'static,
        G: Future<Output = anyhow::Result<Option<T>>> + Send,
    {
        let name = name.to_owned();
        let is_ready = Arc::new(Notify::new());

        let tx = self.tx.clone();
        let go = self.go.clone();
        let ctx = self.ctx.clone();
        let ready = is_ready.clone();
        tokio::spawn(async move {
            ready.notify_one();
            go.notified().await;
            while !tx.is_closed() {
                let r = f(ctx.clone());
                let r = r.await;
                match r {
                    Ok(Some(x)) => {
                        let r = tx.send(x).await;
                        if r.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::warn!(name=name, reason=?e, "Poll failed");
                    }
                    _ => (),
                }
                tokio::time::sleep(poll).await;
            }
        });
        is_ready.notified().await
    }

    pub fn run(self) -> Self {
        self.go.notify_waiters();
        self
    }

    pub fn stop(&mut self) {
        self.rx.close()
    }

    pub async fn get(&mut self) -> Option<T> {
        self.rx.recv().await
    }
}

#[cfg(test)]
pub mod ex {
    use crate::{
        api::gen::Felt,
        ctx::{Context, Head, Shared},
        db::Storage,
        eth::{EthApi, EthClient},
        seq::SeqClient,
    };

    use super::*;

    #[derive(Clone, Debug)]
    pub enum Event {
        X(u64),
    }

    async fn poll_x(
        ctx: Arc<Mutex<Context<EthClient, SeqClient>>>,
    ) -> anyhow::Result<Option<Event>> {
        let x = {
            let eth = &ctx.lock().await.eth;
            eth.call().await
        };
        Ok(Some(Event::X(x)))
    }

    const ETH_URL: &str = "https://eth.llamarpc.com";
    const SEQ_URL: &str = "https://alpha-mainnet.starknet.io";

    #[tokio::test]
    #[ignore = "this is just a usage sample"]
    async fn example() -> anyhow::Result<()> {
        let eth = EthClient::new(ETH_URL);
        let seq = SeqClient::new(SEQ_URL);
        let shared = Shared {
            head: Head {
                block_number: 42,
                block_hash: Felt::try_new("0x0")?,
            },
        };
        let storage = Storage::new("./target/db");

        let ctx = Context::new(eth, seq, shared, storage);

        let poll = Duration::from_secs(3);
        let src = Source::new(ctx);
        src.add("x", poll_x, poll).await;
        let mut src = src.run();

        while let Some(event) = src.get().await {
            println!("{event:?}");
        }

        Ok(())
    }
}

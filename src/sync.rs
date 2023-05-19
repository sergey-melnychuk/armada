use std::{sync::Arc, time::Duration};

use futures::Future;
use tokio::sync::{mpsc, oneshot::channel, Mutex, Notify};

use crate::{
    api::gen::NumAsHex,
    ctx::Context,
    eth::{self, EthApi},
    seq::SeqApi,
    util::{is_open, Waiter},
};

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

    pub fn ctx(&self) -> Arc<Mutex<C>> {
        self.ctx.clone()
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
            tokio::time::sleep(poll).await;
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

pub async fn sync<ETH, SEQ, F, R>(source: Source<Event, Context<ETH, SEQ>>, handler: F) -> Waiter
where
    ETH: EthApi + Send + Sync + Clone + 'static,
    SEQ: SeqApi + Send + Sync + Clone + 'static,
    F: Fn(Arc<Mutex<Context<ETH, SEQ>>>, Event) -> R + Copy + Send + Sync + 'static,
    R: Future<Output = ()> + Send + 'static,
{
    let delay = source.ctx().lock().await.config.poll_delay;

    let (tx, mut rx) = channel::<()>();
    let jh = tokio::spawn(async move {
        let mut source = source.run();

        while let Some(event) = source.get().await {
            let ctx = source.ctx();
            tokio::spawn(async move {
                handler(ctx, event).await;
            });
            if !is_open(&mut rx) {
                break;
            }
        }
        tokio::time::sleep(delay).await;
    });

    Waiter::new(jh, tx)
}

#[derive(Debug)]
pub enum Event {
    Ethereum(eth::State),
    Head(u64, NumAsHex),
    Block(crate::api::gen::BlockWithTxs),
    Pending(crate::api::gen::PendingBlockWithTxs),
    Latest(crate::api::gen::BlockWithTxs),
    Uptime(u64),
}

pub async fn handler<ETH, SEQ>(_ctx: Arc<Mutex<Context<ETH, SEQ>>>, event: Event)
where
    ETH: EthApi + Send + Sync + Clone + 'static,
    SEQ: SeqApi + Send + Sync + Clone + 'static,
{
    #[allow(clippy::single_match)] // TODO: remove
    match event {
        Event::Uptime(seconds) => {
            tracing::info!(seconds, "uptime");
        }
        _ => {}
    }
}

pub async fn poll_uptime<ETH, SEQ>(
    ctx: Arc<Mutex<Context<ETH, SEQ>>>,
) -> anyhow::Result<Option<Event>>
where
    ETH: EthApi + Send + Sync + Clone + 'static,
    SEQ: SeqApi + Send + Sync + Clone + 'static,
{
    let instant = ctx.lock().await.since;
    let seconds = instant.elapsed().as_secs();
    Ok(Some(Event::Uptime(seconds)))
}

pub async fn poll_eth<ETH, SEQ>(ctx: Arc<Mutex<Context<ETH, SEQ>>>) -> anyhow::Result<Option<Event>>
where
    ETH: EthApi + Send + Sync + Clone + 'static,
    SEQ: SeqApi + Send + Sync + Clone + 'static,
{
    let addr = ctx.lock().await.config.ethereum_contract_address.clone();
    let state = ctx.lock().await.eth.get_state(&addr).await?;
    Ok(Some(Event::Ethereum(state)))
}

pub async fn poll_seq<ETH, SEQ>(ctx: Arc<Mutex<Context<ETH, SEQ>>>) -> anyhow::Result<Option<Event>>
where
    ETH: EthApi + Send + Sync + Clone + 'static,
    SEQ: SeqApi + Send + Sync + Clone + 'static,
{
    let latest = match ctx.lock().await.seq.get_latest_block().await? {
        Some(block) => block,
        None => return Ok(None),
    };

    let block_number = *latest.block_header.block_number.as_ref() as u64;
    let block_hash = NumAsHex::try_new(latest.block_header.block_hash.0.as_ref()).unwrap();

    tracing::info!(
        number = block_number,
        hash = block_hash.as_ref(),
        "Latest block"
    );

    Ok(Some(Event::Head(block_number, block_hash)))
}

use std::{sync::Arc, time::Duration};

use futures::Future;
use tokio::sync::{mpsc, oneshot::channel, Mutex, Notify};

use crate::{
    api::gen::{BlockWithTxs, Felt},
    ctx::Context,
    db::Storage,
    eth::{self, EthApi},
    seq::SeqApi,
    util::{is_open, Waiter, U256, U64},
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

    pub fn tx(&self) -> mpsc::Sender<T> {
        self.tx.clone()
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
    R: Future<Output = anyhow::Result<Vec<Event>>> + Send + 'static,
{
    let delay = source.ctx().lock().await.config.poll_delay;

    let (tx, mut rx) = channel::<()>();
    let jh = tokio::spawn(async move {
        let mut source = source.run();

        while let Some(event) = source.get().await {
            let ctx = source.ctx();
            let tx = source.tx();
            tokio::spawn(async move {
                match handler(ctx, event.clone()).await {
                    Ok(events) => {
                        for event in events {
                            tx.send(event).await.ok();
                        }
                    }
                    Err(e) => {
                        tracing::error!(?event, reason=?e, "Handler failed");
                        tokio::spawn(async move {
                            tokio::time::sleep(10 * delay).await;
                            tx.send(event).await.ok();
                        });
                    }
                }
            });
            if !is_open(&mut rx) {
                break;
            }
        }
        tokio::time::sleep(delay).await;
    });

    Waiter::new(jh, tx)
}

#[derive(Clone, Debug)]
pub enum Event {
    Ethereum(eth::State),
    Head(u64, Felt),
    PullBlock(Felt),
    PurgeBlock(u64, Felt),
    Uptime(u64), // TODO: remove (it was added just as an example)
}

pub async fn pull_block<SEQ: SeqApi + Send + Sync + Clone + 'static>(
    seq: &SEQ,
    hash: Felt,
) -> anyhow::Result<BlockWithTxs> {
    seq.get_block_by_hash(hash.as_ref()).await
}

pub async fn save_block(
    db: &mut Storage,
    hash: Felt,
    block: BlockWithTxs,
) -> anyhow::Result<Option<Event>> {
    use crate::db::Repo;
    use yakvdb::typed::DB;

    let number = *block.block_header.block_number.as_ref() as u64;
    tokio::task::block_in_place(|| db.blocks.put(hash.as_ref(), block.clone()))?;

    let key = U64::from_u64(number);
    let val = U256::from_hex(hash.as_ref())?;
    db.blocks_index.write().await.insert(&key, val)?;

    let parent_hash = block.block_header.parent_hash.0;
    tracing::debug!(hash = parent_hash.as_ref(), "Parent block");

    let saved_parent_hash = db
        .blocks_index
        .read()
        .await
        .lookup(&U64::from_u64(number - 1))?;
    if saved_parent_hash.is_none() {
        return Ok(Some(Event::PullBlock(parent_hash)));
    }

    let saved_parent_hash = saved_parent_hash.unwrap();
    if parent_hash.as_ref() != &saved_parent_hash.into_str() {
        // A reorg is detected, Nth block's parent_hash is different from stored (N-1)th block hash.
        // TODO: "unsave" saved `number-1` block and pull the correct one instead of it: `parent_hash`.
        return Ok(Some(Event::PurgeBlock(number - 1, parent_hash)));
    }

    Ok(None)
}

pub async fn purge_block(
    _db: &mut Storage,
    _number: u64,
    hash: Felt,
) -> anyhow::Result<Option<Event>> {
    // TODO: unsave all changes related to the block
    Ok(Some(Event::PullBlock(hash)))
}

pub async fn handler<ETH, SEQ>(
    ctx: Arc<Mutex<Context<ETH, SEQ>>>,
    event: Event,
) -> anyhow::Result<Vec<Event>>
where
    ETH: EthApi + Send + Sync + Clone + 'static,
    SEQ: SeqApi + Send + Sync + Clone + 'static,
{
    tracing::debug!(?event, "Handling");
    match event {
        Event::Uptime(seconds) => {
            if seconds % 60 == 0 {
                tracing::info!(seconds, "uptime");
            }
        }
        Event::PullBlock(hash) => {
            tracing::info!(hash = hash.as_ref(), "Pulling block");
            let block = {
                let seq = &ctx.lock().await.seq;
                pull_block(seq, hash.clone()).await?
            };
            tracing::info!(
                number = block.block_header.block_number.as_ref(),
                hash = block.block_header.block_hash.0.as_ref(),
                "Block pulled"
            );

            let maybe_event = {
                let db = &mut ctx.lock().await.db;
                save_block(db, hash, block).await?
            };

            if let Some(event) = maybe_event {
                return Ok(vec![event]);
            }
        }
        Event::PurgeBlock(number, hash) => {
            tracing::info!(number, hash = hash.as_ref(), "Purging block");
            let db = &mut ctx.lock().await.db;
            let maybe_event = purge_block(db, number, hash).await?;
            if let Some(event) = maybe_event {
                return Ok(vec![event]);
            }
        }
        Event::Head(number, hash) => {
            tracing::info!(number, hash = hash.as_ref(), "L2 head");
        }
        Event::Ethereum(state) => {
            let number = state.state_block_number;
            let hash = state.state_block_hash.as_ref();
            tracing::info!(number, hash, "L1 head");
        }
    }

    Ok(vec![])
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
    use crate::db::Repo;
    let latest = ctx.lock().await.seq.get_latest_block().await?;

    let block_number = *latest.block_header.block_number.as_ref() as u64;
    let block_hash = latest.block_header.block_hash.0.clone();

    tracing::info!(number = block_number, hash = block_hash.as_ref(), "L2 head");

    let block_exists = ctx.lock().await.db.blocks.has(block_hash.as_ref())?;
    if !block_exists {
        let parent_hash = latest.block_header.parent_hash.0.clone();
        Ok(Some(Event::PullBlock(parent_hash)))
    } else {
        Ok(Some(Event::Head(block_number, block_hash)))
    }
}

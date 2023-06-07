use std::collections::HashSet;
use std::{sync::Arc, time::Duration};

use futures::Future;
use tokio::sync::{mpsc, oneshot::channel, Mutex, Notify};

use crate::api::gen::BlockStatus;
use crate::db::{AddressAndNumber, AddressWithKeyAndNumber, Repo};
use crate::{
    api::gen::{BlockWithTxs, Felt},
    ctx::Context,
    db::{BlockAndIndex, Storage},
    eth::{self, EthApi},
    seq::{dto, SeqApi},
    util::{is_open, tx_hash, Waiter, U256, U64},
};
use yakvdb::typed::DB;

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
            tokio::time::sleep(poll / 10).await;
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
    ETH: EthApi,
    SEQ: SeqApi,
    F: Fn(Arc<Mutex<Context<ETH, SEQ>>>, Event) -> R + Copy + Send + Sync + 'static,
    R: Future<Output = anyhow::Result<Vec<Event>>> + Send + 'static,
{
    let delay = source.ctx().lock().await.config.src_poll_delay;

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
    });

    Waiter::new(jh, tx)
}

#[derive(Clone, Debug)]
pub enum Event {
    Ethereum(eth::State),
    Head(u64, Felt),
    PullBlock(Felt),
    PurgeBlock(u64, Felt),
    Uptime { seconds: u64 },
}

pub async fn fetch_block<SEQ, ETH>(
    ctx: Arc<Mutex<Context<ETH, SEQ>>>,
    hash: &Felt,
) -> anyhow::Result<BlockWithTxs>
where
    SEQ: SeqApi,
    ETH: EthApi,
{
    let block = {
        let seq = &ctx.lock().await.seq;
        seq.get_block_by_hash(hash.as_ref()).await?
    };
    let block_number = *block.block_header.block_number.as_ref() as u64;
    let block_hash = block.block_header.block_hash.0.clone();

    let block = match &block.status {
        BlockStatus::AcceptedOnL1 | BlockStatus::AcceptedOnL2 => block,
        status => {
            // Double-check block status (must be ACCEPTED_ON_{L1,L2}) and retry.
            // Some blocks are returned with ABORTED status when queried by hash,
            // but then return ACCEPTED_ON_{L1,L2} if queried by the block number.
            //
            // Reproducible with blocks:
            // mainnet:12304/0x7cebd154f03c5f838999351e2a7f5f1346ea161d355155d424e7b4efda52ccd
            // mainnet:12302/0x14d51955b90b1d74e9cf22bf3352c6a7d13036203c65da7bee77b9d7a5f6ab7
            // mainnet:12297/0x3dc5e7fd184af0c07d1a7542d93d0ba933dc355502fa1336ab252589c5b5a38
            // mainnet:12296/0x5f28108855e545894b750836148d1e65f200c159ad52230155b74b14156a477
            tracing::warn!(number = block_number, hash = block_hash.as_ref(), status=?status, "Unexpected block status");
            let block = {
                let seq = &ctx.lock().await.seq;
                seq.get_block_by_number(block_number).await?
            };
            match &block.status {
                BlockStatus::AcceptedOnL1 | BlockStatus::AcceptedOnL2 => {
                    tracing::warn!(number = block_number, hash = block_hash.as_ref(), status=?block.status, "Block fetch retry OK");
                }
                _ => {
                    tracing::warn!(number = block_number, hash = block_hash.as_ref(), status=?block.status, "Block fetch retry failed");
                }
            }
            block
        }
    };

    Ok(block)
}

pub async fn pull_block<SEQ, ETH>(
    ctx: Arc<Mutex<Context<ETH, SEQ>>>,
    hash: Felt,
    events: &mut Vec<Event>,
) -> anyhow::Result<u64>
where
    SEQ: SeqApi,
    ETH: EthApi,
{
    tracing::debug!(hash = hash.as_ref(), "Pulling block");

    let block = fetch_block(ctx.clone(), &hash).await?;
    let block_number = *block.block_header.block_number.as_ref() as u64;
    let block_hash = block.block_header.block_hash.0.clone();

    if let Some(event) = {
        let db = &mut ctx.lock().await.db;
        save_block(db, hash.clone(), block).await?
    } {
        events.push(event);
    }

    tracing::debug!(
        number = block_number,
        hash = block_hash.as_ref(),
        "Block saved"
    );

    let state = {
        let seq = &ctx.lock().await.seq;
        seq.get_state_by_hash(hash.as_ref()).await?
    };

    let handle = {
        let ctx = ctx.clone();
        let classes = get_classes(&state)
            .map(|(_, hash)| hash.as_ref().to_string())
            .collect::<HashSet<_>>();
        tokio::spawn(async move {
            for hash in classes {
                if ctx.lock().await.db.classes.has(&hash).await? {
                    continue;
                }
                let class = ctx.lock().await.seq.get_class_by_hash(&hash).await?;
                ctx.lock().await.db.classes.put(&hash, class).await?;
                tracing::debug!(hash, "Class saved");
            }
            Ok::<(), anyhow::Error>(())
        })
    };

    {
        let db = &mut ctx.lock().await.db;
        save_state(db, hash.clone(), block_number, state).await?
    };

    handle.await??;

    tracing::debug!(
        number = block_number,
        hash = block_hash.as_ref(),
        "State saved"
    );

    Ok(block_number)
}

// TODO: avoid function-scoped lock on Storage
pub async fn save_block(
    db: &mut Storage,
    hash: Felt,
    block: BlockWithTxs,
) -> anyhow::Result<Option<Event>> {
    let number = *block.block_header.block_number.as_ref() as u64;
    db.blocks.put(hash.as_ref(), block.clone()).await?;

    let key = U64::from_u64(number);
    let val = U256::from_hex(hash.as_ref())?;
    db.blocks_index.write().await.insert(&key, val)?;

    // TODO: spawn
    for (idx, tx) in block.block_body_with_txs.transactions.iter().enumerate() {
        let index = U64::from_u64(idx as u64);
        let block = U256::from_hex(hash.as_ref()).unwrap();

        let val = BlockAndIndex::from(block, index);
        let key = U256::from_hex(tx_hash(tx).as_ref())?;
        db.txs_index.write().await.insert(&key, val)?;
        tracing::debug!(hash = key.into_str(), "TX saved");
    }

    // TODO: spawn
    for receipt in &block.receipts {
        for event in &receipt.events {
            let addr = &event.from_address.0;
            let keys = &event.event_content.keys;
            for key in keys {
                let address = U256::from_hex(addr.as_ref()).unwrap();
                let event_key = U256::from_hex(key.as_ref()).unwrap();
                let num = U64::from_u64(number);

                let key = AddressWithKeyAndNumber::from(address, event_key.clone(), num);
                let val = U64::from_u64(receipt.transaction_index as u64);
                db.events_index.write().await.insert(&key, val)?;
                tracing::debug!(
                    address = addr.as_ref(),
                    key = event_key.into_str(),
                    "Event saved"
                );
            }
        }
    }

    if number == 0 {
        // Stop if a genesis block is reached
        return Ok(None);
    }

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
        tracing::warn!(
            existing = saved_parent_hash.into_str(),
            received = parent_hash.as_ref(),
            "Reorg detected"
        );
        // A reorg is detected, Nth block's parent_hash is different from stored (N-1)th block hash.
        // TODO: "unsave" saved `number-1` block and pull the correct one instead of it: `parent_hash`.
        return Ok(Some(Event::PurgeBlock(number - 1, parent_hash)));
    }

    Ok(None)
}

// TODO: avoid function-scoped lock on Storage
pub async fn save_state(
    db: &mut Storage,
    hash: Felt,
    number: u64,
    state: dto::StateUpdate,
) -> anyhow::Result<()> {
    db.states.put(hash.as_ref(), state.clone()).await?;

    // TODO: spawn
    for (addr, nonce) in &state.state_diff.nonces {
        let address = U256::from_hex(addr.as_ref()).unwrap();
        let number = U64::from_u64(number);
        let key = AddressAndNumber::from(address, number);
        let val = U256::from_hex(nonce.as_ref())?;
        db.nonces_index.write().await.insert(&key, val)?;
        tracing::debug!(
            address = key.address().into_str(),
            nonce = nonce.as_ref(),
            "Nonce saved"
        );
    }

    // TODO: spawn
    for (addr, kvs) in &state.state_diff.storage_diffs {
        let address = U256::from_hex(addr.as_ref()).unwrap();
        let number = U64::from_u64(number);
        for kv in kvs {
            let key = U256::from_hex(kv.key.as_ref()).unwrap();
            let val = U256::from_hex(kv.value.as_ref()).unwrap();
            let item = AddressWithKeyAndNumber::from(address.clone(), key, number.clone());
            db.states_index.write().await.insert(&item, val)?;
            tracing::debug!(
                address = addr.as_ref(),
                key = kv.key.as_ref(),
                val = kv.value.as_ref(),
                "Store saved"
            );
        }
    }

    // TODO: spawn
    for (addr, hash) in get_classes(&state) {
        let address = U256::from_hex(addr.as_ref()).unwrap();
        let number = U64::from_u64(number);
        let key = AddressAndNumber::from(address, number);
        let val = U256::from_hex(hash.as_ref())?;
        db.classes_index.write().await.insert(&key, val)?;
        tracing::debug!(
            address = key.address().into_str(),
            hash = hash.as_ref(),
            "Class assigned"
        );
    }

    // TODO: how to handle [old_]declared_contracts?
    Ok(())
}

pub fn get_classes(state: &dto::StateUpdate) -> impl Iterator<Item = (&Felt, &Felt)> + '_ {
    state
        .state_diff
        .deployed_contracts
        .iter()
        .map(|deployed| (&deployed.address, &deployed.class_hash))
        .chain(
            state
                .state_diff
                .replaced_classes
                .iter()
                .map(|replaced| (&replaced.address, &replaced.class_hash)),
        )
}

// TODO: avoid function-scoped lock on Storage
pub async fn purge_block(
    _db: &mut Storage,
    _number: u64,
    hash: Felt,
    events: &mut Vec<Event>,
) -> anyhow::Result<()> {
    // TODO: unsave all changes related to the block
    // Currently re-pulling the block will restore the chain integrity,
    // but indexed data from "purged" block will remain available.

    events.push(Event::PullBlock(hash));
    Ok(())
}

pub async fn handler<ETH, SEQ>(
    ctx: Arc<Mutex<Context<ETH, SEQ>>>,
    event: Event,
) -> anyhow::Result<Vec<Event>>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    tracing::debug!(?event, "Handling");
    let mut events = Vec::new();
    match event {
        Event::Uptime { seconds } => {
            if seconds > 0 && seconds % 60 == 0 {
                tracing::info!(seconds, "uptime");
            }
        }
        Event::PullBlock(hash) => {
            let number = pull_block(ctx.clone(), hash.clone(), &mut events).await?;
            tracing::info!(number, hash = hash.as_ref(), "Block done");
            {
                let ctx = ctx.lock().await;
                let sync = &mut ctx.shared.lock().await.sync;
                sync.lo = sync.lo.map(|lo| number.min(lo)).or(Some(number));
                sync.hi = sync.hi.map(|hi| number.max(hi)).or(Some(number));
            }
        }
        Event::PurgeBlock(number, hash) => {
            let db = &mut ctx.lock().await.db;
            purge_block(db, number, hash.clone(), &mut events).await?;
            tracing::warn!(number, hash = hash.as_ref(), "Block purged");
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

    Ok(events)
}

pub async fn poll_uptime<ETH, SEQ>(
    ctx: Arc<Mutex<Context<ETH, SEQ>>>,
) -> anyhow::Result<Option<Event>>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    let instant = ctx.lock().await.since;
    let seconds = instant.elapsed().as_secs();
    Ok(Some(Event::Uptime { seconds }))
}

pub async fn poll_eth<ETH, SEQ>(ctx: Arc<Mutex<Context<ETH, SEQ>>>) -> anyhow::Result<Option<Event>>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    let addr = ctx.lock().await.config.ethereum_contract_address.clone();
    let state = ctx.lock().await.eth.get_state(&addr).await?;
    Ok(Some(Event::Ethereum(state)))
}

pub async fn poll_seq<ETH, SEQ>(ctx: Arc<Mutex<Context<ETH, SEQ>>>) -> anyhow::Result<Option<Event>>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    let latest = ctx.lock().await.seq.get_latest_block().await?;

    let block_number = *latest.block_header.block_number.as_ref() as u64;
    let block_hash = latest.block_header.block_hash.0.clone();

    tracing::info!(
        number = block_number,
        hash = block_hash.as_ref(),
        "Latest block"
    );

    let block_exists = ctx.lock().await.db.blocks.has(block_hash.as_ref()).await?;
    if !block_exists {
        Ok(Some(Event::PullBlock(block_hash)))
    } else {
        Ok(Some(Event::Head(block_number, block_hash)))
    }
}

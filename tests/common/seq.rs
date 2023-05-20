use std::sync::Arc;

use armada::{api::gen::BlockWithTxs, seq::SeqApi};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct TestSeq {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Default)]
struct Inner {
    latest: Option<BlockWithTxs>,
}

impl TestSeq {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::default())),
        }
    }

    pub async fn set_latest(&self, latest: BlockWithTxs) {
        self.inner().await.latest = Some(latest);
    }

    pub async fn reset_latest(&self) {
        self.inner().await.latest = None;
    }

    async fn inner(&self) -> MutexGuard<Inner> {
        self.inner.lock().await
    }
}

#[async_trait::async_trait]
impl SeqApi for TestSeq {
    async fn get_block_by_number(
        &self,
        _block_number: u64,
    ) -> anyhow::Result<armada::api::gen::BlockWithTxs> {
        anyhow::bail!("Block not found");
    }

    async fn get_block_by_hash(
        &self,
        _block_hash: &str,
    ) -> anyhow::Result<armada::api::gen::BlockWithTxs> {
        anyhow::bail!("Block not found");
    }

    async fn get_latest_block(&self) -> anyhow::Result<armada::api::gen::BlockWithTxs> {
        if let Some(latest) = self.inner().await.latest.as_ref() {
            return Ok(latest.clone());
        }
        anyhow::bail!("Block not found");
    }

    async fn get_pending_block(
        &self,
    ) -> anyhow::Result<armada::api::gen::PendingBlockWithTxs> {
        anyhow::bail!("Block not found");
    }

    async fn get_state_by_number(
        &self,
        _block_number: u64,
    ) -> anyhow::Result<armada::seq::dto::StateUpdate> {
        anyhow::bail!("State Update not found");
    }

    async fn get_state_by_hash(
        &self,
        _block_hash: &str,
    ) -> anyhow::Result<armada::seq::dto::StateUpdate> {
        anyhow::bail!("State Update not found");
    }
}

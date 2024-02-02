use std::sync::Arc;

use armada::{api::gen::BlockWithTxs, seq::SeqApi};
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

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

    pub async fn latest(&self) -> MappedMutexGuard<Option<BlockWithTxs>> {
        MutexGuard::map(self.inner.lock().await, |inner| &mut inner.latest)
    }
}

#[async_trait::async_trait]
impl SeqApi for TestSeq {
    async fn get_block_by_number(
        &self,
        _block_number: u64,
    ) -> anyhow::Result<armada::api::gen::BlockWithTxs> {
        Err(anyhow::anyhow!("Block not found"))
    }

    async fn get_block_by_hash(
        &self,
        _block_hash: &str,
    ) -> anyhow::Result<armada::api::gen::BlockWithTxs> {
        Err(anyhow::anyhow!("Block not found"))
    }

    async fn get_latest_block(
        &self,
    ) -> anyhow::Result<armada::api::gen::BlockWithTxs> {
        let latest = self.latest().await;
        if let Some(latest) = latest.as_ref() {
            return Ok(latest.clone());
        }
        Err(anyhow::anyhow!("Block not found"))
    }

    async fn get_pending_block(
        &self,
    ) -> anyhow::Result<armada::api::gen::PendingBlockWithTxs> {
        Err(anyhow::anyhow!("Block not found"))
    }

    async fn get_state_by_number(
        &self,
        _block_number: u64,
    ) -> anyhow::Result<armada::seq::dto::StateUpdate> {
        Err(anyhow::anyhow!("State Update not found"))
    }

    async fn get_state_by_hash(
        &self,
        _block_hash: &str,
    ) -> anyhow::Result<armada::seq::dto::StateUpdate> {
        Err(anyhow::anyhow!("State Update not found"))
    }

    async fn get_class_by_hash(
        &self,
        _block_hash: &str,
    ) -> anyhow::Result<armada::seq::dto::Class> {
        Err(anyhow::anyhow!("Class not found"))
    }
}

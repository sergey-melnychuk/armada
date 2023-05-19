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
    ) -> anyhow::Result<Option<armada::api::gen::BlockWithTxs>> {
        Ok(None)
    }

    async fn get_block_by_hash(
        &self,
        _block_hash: &str,
    ) -> anyhow::Result<Option<armada::api::gen::BlockWithTxs>> {
        Ok(None)
    }

    async fn get_latest_block(&self) -> anyhow::Result<Option<armada::api::gen::BlockWithTxs>> {
        Ok(self.inner().await.latest.clone())
    }

    async fn get_pending_block(
        &self,
    ) -> anyhow::Result<Option<armada::api::gen::PendingBlockWithTxs>> {
        Ok(None)
    }
}

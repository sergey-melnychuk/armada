use std::sync::Arc;

use armada::seq::SeqApi;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct TestSeq {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Default)]
struct Inner {
    call_response: u64,
}

impl TestSeq {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::default())),
        }
    }

    async fn inner(&self) -> MutexGuard<Inner> {
        self.inner.lock().await
    }

    pub async fn set_call_response(&mut self, x: u64) {
        let mut inner = self.inner().await;
        inner.call_response = x;
    }
}

#[async_trait::async_trait]
impl SeqApi for TestSeq {
    async fn call(&self) -> u64 {
        self.inner().await.call_response
    }
}

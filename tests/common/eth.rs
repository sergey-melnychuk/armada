use std::sync::Arc;

use armada::eth::EthApi;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct TestEth {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Default)]
struct Inner {
    state: Option<armada::eth::State>,
}

impl TestEth {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::default())),
        }
    }

    pub async fn set_state(&self, state: armada::eth::State) {
        self.inner().await.state = Some(state);
    }

    pub async fn reset_state(&self) {
        self.inner().await.state = None;
    }

    async fn inner(&self) -> MutexGuard<Inner> {
        self.inner.lock().await
    }
}

#[async_trait::async_trait]
impl EthApi for TestEth {
    async fn get_state(&self, _address: &str) -> anyhow::Result<armada::eth::State> {
        if let Some(state) = self.inner().await.state.as_ref() {
            Ok(state.clone())
        } else {
            anyhow::bail!("Failed to fetch ethereum contract state");
        }
    }
}

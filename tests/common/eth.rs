use std::sync::Arc;

use armada::eth::EthApi;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

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

    pub async fn state(&self) -> MappedMutexGuard<Option<armada::eth::State>> {
        MutexGuard::map(self.inner.lock().await, |inner| &mut inner.state)
    }
}

#[async_trait::async_trait]
impl EthApi for TestEth {
    async fn get_state(
        &self,
        _address: &str,
    ) -> anyhow::Result<armada::eth::State> {
        let state = self.state().await;
        if let Some(state) = state.as_ref() {
            Ok(state.clone())
        } else {
            anyhow::bail!("Failed to fetch ethereum contract state");
        }
    }
}

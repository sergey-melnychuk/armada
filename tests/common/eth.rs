use armada::eth::EthApi;

#[derive(Clone)]
pub struct TestEth;

#[async_trait::async_trait]
impl EthApi for TestEth {
    async fn call(&self) -> u64 {
        42
    }
}

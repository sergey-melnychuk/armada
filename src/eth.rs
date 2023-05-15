// TODO: Ethereum DTO, API

#[async_trait::async_trait]
pub trait EthApi {
    async fn call(&self) -> u64;
}

pub struct EthClient {}

#[async_trait::async_trait]
impl EthApi for EthClient {
    async fn call(&self) -> u64 {
        42
    }
}

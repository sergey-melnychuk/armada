// TODO: Ethereum DTO, API

#[async_trait::async_trait]
pub trait EthApi {
    async fn call(&self) -> u64;
}

#[async_trait::async_trait]
impl EthApi for EthClient {
    async fn call(&self) -> u64 {
        42
    }
}

#[derive(Clone)]
pub struct EthClient {
    url: String,
}

impl EthClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

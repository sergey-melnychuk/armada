// TODO: Ethereum DTO, API

#[async_trait::async_trait]
pub trait EthApi {
    // TODO: call: address, params, result

    // TODO: remove this method
    async fn test_call(&self) -> u64;
}

#[async_trait::async_trait]
impl EthApi for EthClient {
    async fn test_call(&self) -> u64 {
        42
    }
}

#[derive(Clone)]
pub struct EthClient {
    http: reqwest::Client,
    url: String,
}

impl EthClient {
    pub fn new(url: &str) -> Self {
        let http = reqwest::ClientBuilder::new()
            .build()
            .expect("Failed to create HTTP client");
        Self {
            http,
            url: url.to_string(),
        }
    }
}

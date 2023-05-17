// TODO: sequencer DTO, API

#[async_trait::async_trait]
pub trait SeqApi {
    async fn call(&self) -> u64;
}

#[async_trait::async_trait]
impl SeqApi for SeqClient {
    async fn call(&self) -> u64 {
        42
    }
}

#[derive(Clone)]
pub struct SeqClient {
    url: String,
}

impl SeqClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

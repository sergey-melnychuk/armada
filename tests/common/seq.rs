use armada::seq::SeqApi;

#[derive(Clone)]
pub struct TestSeq;

#[async_trait::async_trait]
impl SeqApi for TestSeq {
    async fn call(&self) -> u64 {
        42
    }
}

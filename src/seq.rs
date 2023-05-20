use serde::de::DeserializeOwned;

use crate::{
    api::gen::{BlockWithTxs, PendingBlockWithTxs},
    util::{patch_block, patch_pending_block},
};

pub mod dto {
    use serde::{Deserialize, Serialize};

    use crate::api::gen::Felt;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StateUpdate {
        pub block_hash: Felt,
        pub new_root: Felt,
        pub old_root: Felt,
        pub state_diff: StorageDiff,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageDiff {
        #[serde(with = "tuple_vec_map")]
        pub storage_diffs: Vec<(Felt, Vec<KeyValue>)>,
        #[serde(with = "tuple_vec_map")]
        pub nonces: Vec<(Felt, Felt)>,
        pub deployed_contracts: Vec<DeployedContract>,
        pub old_declared_contracts: Vec<serde_json::Value>, // TODO: match to a DTO
        pub declared_classes: Vec<serde_json::Value>,       // TODO: match to a DTO
        pub replaced_classes: Vec<serde_json::Value>,       // TODO: match to a DTO
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyValue {
        pub key: Felt,
        pub value: Felt,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeployedContract {
        pub address: Felt,
        pub class_hash: Felt,
    }
}

#[async_trait::async_trait]
pub trait SeqApi {
    async fn get_block_by_number(&self, block_number: u64) -> anyhow::Result<BlockWithTxs>;
    async fn get_block_by_hash(&self, block_hash: &str) -> anyhow::Result<BlockWithTxs>;
    async fn get_latest_block(&self) -> anyhow::Result<BlockWithTxs>;
    async fn get_pending_block(&self) -> anyhow::Result<PendingBlockWithTxs>;

    async fn get_state_by_number(&self, block_number: u64) -> anyhow::Result<dto::StateUpdate>;
    async fn get_state_by_hash(&self, block_hash: &str) -> anyhow::Result<dto::StateUpdate>;

    // TODO: get_contract
}

#[async_trait::async_trait]
impl SeqApi for SeqClient {
    async fn get_block_by_number(&self, block_number: u64) -> anyhow::Result<BlockWithTxs> {
        self.get_block(&format!("blockNumber={block_number}"), patch_block)
            .await
    }

    async fn get_block_by_hash(&self, block_hash: &str) -> anyhow::Result<BlockWithTxs> {
        self.get_block(&format!("blockHash={}", block_hash), patch_block)
            .await
    }

    async fn get_latest_block(&self) -> anyhow::Result<BlockWithTxs> {
        self.get_block("blockNumber=latest", patch_block).await
    }

    async fn get_pending_block(&self) -> anyhow::Result<PendingBlockWithTxs> {
        self.get_block("blockNumber=pending", patch_pending_block)
            .await
    }

    async fn get_state_by_number(&self, block_number: u64) -> anyhow::Result<dto::StateUpdate> {
        self.get_state(&format!("blockNumber={block_number}")).await
    }

    async fn get_state_by_hash(&self, block_hash: &str) -> anyhow::Result<dto::StateUpdate> {
        self.get_state(&format!("blockHash={}", block_hash)).await
    }
}

#[derive(Clone)]
pub struct SeqClient {
    http: reqwest::Client,
    url: String,
}

impl SeqClient {
    pub fn new(url: &str) -> Self {
        let http = reqwest::ClientBuilder::new()
            .build()
            .expect("Failed to create HTTP client");
        Self {
            http,
            url: url.to_string(),
        }
    }

    async fn get_block<T: DeserializeOwned>(
        &self,
        arg: &str,
        map: fn(serde_json::Value) -> serde_json::Value,
    ) -> anyhow::Result<T> {
        let url = format!("{}/feeder_gateway/get_block?{arg}", self.url);
        let value: serde_json::Value = self.http.get(&url).send().await?.json().await?;
        let block = serde_json::from_value(map(value))?;
        Ok(block)
    }

    async fn get_state<T: DeserializeOwned>(&self, arg: &str) -> anyhow::Result<T> {
        let url = format!("{}/feeder_gateway/get_state_update?{arg}", self.url);
        let state = self.http.get(&url).send().await?.json().await?;
        Ok(state)
    }
}

#[cfg(test)]
mod tests {

    mod goerli {
        use super::super::*;

        const URL: &str = "https://alpha4.starknet.io";

        const NUMBER: u64 = 805543;
        const HASH: &str = "0x212440a93e19eb76b3da51b13317152d9412913385d7004079c5b0ff6b224af";

        #[tokio::test]
        async fn test_block_by_number() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_block_by_number(NUMBER).await?;
            assert_eq!(block.block_header.block_hash.0.as_ref(), HASH);
            Ok(())
        }

        #[tokio::test]
        async fn test_block_by_hash() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_block_by_hash(HASH).await?;
            assert_eq!(block.block_header.block_number.as_ref(), &(NUMBER as i64));
            Ok(())
        }

        #[tokio::test]
        async fn test_block_latest() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_latest_block().await?;
            assert!(block.block_header.block_number.as_ref() > &(NUMBER as i64));
            Ok(())
        }

        #[tokio::test]
        async fn test_block_pending() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_pending_block().await?;
            assert!(!block.block_body_with_txs.transactions.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn test_state_by_hash() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let state = seq.get_state_by_hash(HASH).await?;
            assert_eq!(state.block_hash.as_ref(), HASH);
            Ok(())
        }

        #[tokio::test]
        async fn test_state_by_number() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let state = seq.get_state_by_number(NUMBER).await?;
            assert_eq!(state.block_hash.as_ref(), HASH);
            Ok(())
        }
    }
}

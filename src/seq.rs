use serde::de::DeserializeOwned;

use crate::{
    api::gen::{BlockWithTxs, PendingBlockWithTxs},
    util::{patch_block, patch_pending_block},
};

// TODO: sequencer DTO, API

#[async_trait::async_trait]
pub trait SeqApi {
    async fn get_block_by_number(&self, block_number: u64) -> anyhow::Result<Option<BlockWithTxs>>;
    async fn get_block_by_hash(&self, block_hash: &str) -> anyhow::Result<Option<BlockWithTxs>>;
    async fn get_latest_block(&self) -> anyhow::Result<Option<BlockWithTxs>>;
    async fn get_pending_block(&self) -> anyhow::Result<Option<PendingBlockWithTxs>>;

    // TODO: get_state_update
    // TODO: get_contract
}

#[async_trait::async_trait]
impl SeqApi for SeqClient {
    async fn get_block_by_number(&self, block_number: u64) -> anyhow::Result<Option<BlockWithTxs>> {
        self.get_block(&format!("blockNumber={block_number}"), patch_block)
            .await
    }

    async fn get_block_by_hash(&self, block_hash: &str) -> anyhow::Result<Option<BlockWithTxs>> {
        self.get_block(&format!("blockHash={}", block_hash), patch_block)
            .await
    }

    async fn get_latest_block(&self) -> anyhow::Result<Option<BlockWithTxs>> {
        self.get_block("blockNumber=latest", patch_block).await
    }

    async fn get_pending_block(&self) -> anyhow::Result<Option<PendingBlockWithTxs>> {
        self.get_block("blockNumber=pending", patch_pending_block)
            .await
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
    ) -> anyhow::Result<Option<T>> {
        let url = format!("{}/feeder_gateway/get_block?{arg}", self.url);
        let value: serde_json::Value = self.http.get(&url).send().await?.json().await?;
        let block = serde_json::from_value(map(value))?;
        Ok(Some(block))
    }
}

#[cfg(test)]
mod tests {

    // TODO: mainnet: use or remove?
    // const SEQ_URL: &str = "https://alpha-mainnet.starknet.io";
    // const ETH_URL: &str = "https://eth.llamarpc.com";

    mod goerli {
        use super::super::*;

        const URL: &str = "https://alpha4.starknet.io";

        const NUMBER: u64 = 805543;
        const HASH: &str = "0x212440a93e19eb76b3da51b13317152d9412913385d7004079c5b0ff6b224af";

        #[tokio::test]
        async fn test_block_by_number() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_block_by_number(NUMBER).await?.expect("block");
            assert_eq!(block.block_header.block_hash.0.as_ref(), HASH);
            Ok(())
        }

        #[tokio::test]
        async fn test_block_by_hash() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_block_by_hash(HASH).await?.expect("block");
            assert_eq!(block.block_header.block_number.as_ref(), &(NUMBER as i64));
            Ok(())
        }

        #[tokio::test]
        async fn test_block_latest() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_latest_block().await?.expect("block");
            assert!(block.block_header.block_number.as_ref() > &(NUMBER as i64));
            Ok(())
        }

        #[tokio::test]
        async fn test_block_pending() -> anyhow::Result<()> {
            let seq = SeqClient::new(URL);
            let block = seq.get_pending_block().await?.expect("block");
            assert!(!block.block_body_with_txs.transactions.is_empty());
            Ok(())
        }
    }
}

use std::fs;

use serde_json::json;

mod common;

// curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getBlockWithTxHashes","params":{"block_number":42},"id":1}' http://localhost:9000/rpc/v0.3
mod get_block_with_tx_hashes {
    use armada::api::gen::{BlockWithTxs, GetBlockWithTxHashesResult};
    use armada::db::Repo;

    use super::*;

    #[tokio::test]
    async fn test_existing_block() -> anyhow::Result<()> {
        let json = fs::read_to_string("./etc/805543-block.json")?;
        let block: BlockWithTxs = serde_json::from_str(&json)?;
        let hash = block.block_header.block_hash.0.as_ref().clone();

        let test = common::Test::new().await;
        test.ctx.db.blocks.put(&hash, block)?;

        let res: GetBlockWithTxHashesResult = test
            .rpc(json!({
                "jsonrpc": "2.0",
                "method": "starknet_getBlockWithTxHashes",
                "params": {"block_hash": hash},
                "id": 1
            }))
            .await?;

        let block = match res {
            GetBlockWithTxHashesResult::BlockWithTxHashes(block) => block,
            unexpected => anyhow::bail!("Unexpected variant: {unexpected:?}"),
        };

        assert_eq!(block.block_header.block_hash.0.as_ref(), &hash);
        assert_eq!(block.block_header.block_number.as_ref(), &805543);

        Ok(())
    }
}

mod get_block_with_txs {
    use armada::api::gen::{BlockWithTxs, GetBlockWithTxsResult};
    use armada::db::Repo;

    use super::*;

    #[tokio::test]
    async fn test_existing_block() -> anyhow::Result<()> {
        let json = fs::read_to_string("./etc/805543-block.json")?;
        let block: BlockWithTxs = serde_json::from_str(&json)?;
        let hash = block.block_header.block_hash.0.as_ref().clone();

        let test = common::Test::new().await;
        test.ctx.db.blocks.put(&hash, block)?;

        let res: GetBlockWithTxsResult = test
            .rpc(json!({
                "jsonrpc": "2.0",
                "method": "starknet_getBlockWithTxs",
                "params": {"block_hash": hash},
                "id": 1
            }))
            .await?;

        let block = match res {
            GetBlockWithTxsResult::BlockWithTxs(block) => block,
            unexpected => anyhow::bail!("Unexpected variant: {unexpected:?}"),
        };

        assert_eq!(block.block_header.block_hash.0.as_ref(), &hash);
        assert_eq!(block.block_header.block_number.as_ref(), &805543);

        Ok(())
    }
}

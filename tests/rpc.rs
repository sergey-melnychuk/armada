use armada::{ctx::Context, rpc::gen::GetBlockWithTxHashesResult};
use serde_json::json;

mod common;

mod get_block_with_tx_hashes {
    use armada::db::Storage;

    use super::*;

    #[tokio::test]
    async fn test_genesis_block() -> anyhow::Result<()> {
        let ctx = Context::new(Storage {});

        let test = common::Test::new(ctx).await;

        // curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getBlockWithTxHashes","params":{"block_number":42},"id":1}' http://localhost:9000/rpc/v0.3
        let res: GetBlockWithTxHashesResult = test
            .call(json!({
                "jsonrpc": "2.0",
                "method": "starknet_getBlockWithTxHashes",
                "params": {"block_number": 42},
                "id": 1
            }))
            .await?;

        let block = match res {
            GetBlockWithTxHashesResult::BlockWithTxHashes(block) => block,
            unexpected => anyhow::bail!("Unexpected variant: {unexpected:?}"),
        };

        assert_eq!(block.block_header.block_hash.0.as_ref(), "0x0");
        assert_eq!(block.block_header.block_number.as_ref(), &0);

        Ok(())
    }
}

use std::fs;

use armada::{
    api::gen::{BlockWithTxs, PendingBlockWithTxs},
    util::{patch_block, patch_pending_block},
};

#[test]
fn test_parse_original_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/805543-block.json")?;
    let block: BlockWithTxs = serde_json::from_str(&json)?;
    assert!(!block.block_body_with_txs.transactions.is_empty());
    Ok(())
}

#[test]
fn test_parse_pending_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/pending.json")?;
    let value: serde_json::Value = serde_json::from_str(&json)?;
    let block: PendingBlockWithTxs =
        serde_json::from_value(patch_pending_block(value))?;
    assert!(!block.block_body_with_txs.transactions.is_empty());
    Ok(())
}

#[test]
fn test_parse_latest_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/latest.json")?;
    let value: serde_json::Value = serde_json::from_str(&json)?;
    let block: PendingBlockWithTxs =
        serde_json::from_value(patch_block(value))?;
    assert!(!block.block_body_with_txs.transactions.is_empty());
    Ok(())
}

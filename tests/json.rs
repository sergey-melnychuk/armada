use std::fs;

use armada::api::gen::BlockWithTxs;

#[test]
fn test_parse_patched_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/805543.patched.json")?;
    let block: BlockWithTxs = serde_json::from_str(&json)?;
    assert!(!block.block_body_with_txs.transactions.is_empty());
    Ok(())
}

#[test]
fn test_parse_original_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/805543.json")?;
    let block: BlockWithTxs = serde_json::from_str(&json)?;
    assert!(!block.block_body_with_txs.transactions.is_empty());
    Ok(())
}

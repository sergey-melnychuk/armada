use std::fs;

use armada::api::gen::{BlockWithTxs, InvokeTxn, Txn};

#[test]
fn test_parse_original_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/805543-block.json")?;
    let block: BlockWithTxs = serde_json::from_str(&json)?;
    assert!(!block.block_body_with_txs.transactions.is_empty());
    Ok(())
}

#[test]
#[ignore]
fn test_parse_pending_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/pending.json")?;
    let block: serde_json::Value = serde_json::from_str(&json)?;

    // TODO: unfuck this
    // TXN::INVOKE_TXN::FUNCTION_CALL does not have a "nonce" sometimes in a "pending" block!
    let txs = block["transactions"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    for (i, tx) in txs.into_iter().enumerate() {
        println!("\ntx={i}:");
        let tx: Txn = serde_json::from_value(tx.clone())?;
        println!("{tx:?}");
    }

    Ok(())
}

use std::fs;

use armada::api::gen::BlockWithTxs;

#[test]
fn test_parse_patched_block() -> anyhow::Result<()> {
    let json = fs::read_to_string("./etc/805543.patched.json")?;
    let block: BlockWithTxs = serde_json::from_str(&json)?;
    assert!(block.block_body_with_txs.transactions.len() > 0);
    Ok(())
}

#[test]
#[ignore]
fn test_parse_original_block() -> anyhow::Result<()> {
    /*
    $ diff etc/805543.json etc/805543.patched.json
    3c3
    <   "parent_block_hash": "0x52deecc1c8fb21639a2afec4a5ec4b47c46dfb316add70a978af4b168564ada",
    ---
    >   "parent_hash": "0x52deecc1c8fb21639a2afec4a5ec4b47c46dfb316add70a978af4b168564ada",
    5c5
    <   "state_root": "0x5bb4960857b4035de20390fac6050363fe75d41cbeda76f29865a6dfb998b5",
    ---
    >   "new_root": "0x5bb4960857b4035de20390fac6050363fe75d41cbeda76f29865a6dfb998b5",
    29c29
    <       "type": "INVOKE_FUNCTION"
    ---
    >       "type": "INVOKE"
    */
    let json = fs::read_to_string("./etc/805543.json")?;
    let block: BlockWithTxs = serde_json::from_str(&json)?;
    assert!(block.block_body_with_txs.transactions.len() > 0);
    Ok(())
}

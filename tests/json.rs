use std::fs;

use armada::rpc::gen::BlockWithTxs;

#[test]
fn test_parse_block() {
    let json = fs::read_to_string("./etc/805543.patched.json").expect("json");
    let block: BlockWithTxs = serde_json::from_str(&json).expect("parse");
    assert!(block.block_body_with_txs.transactions.len() > 0);
}

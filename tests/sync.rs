use std::time::Duration;

use armada::{
    api::gen::{BlockWithTxs, NumAsHex},
    sync::{self, Event, Source},
};
use serde::de::DeserializeOwned;

mod common;

#[tokio::test]
async fn test_sync_events() -> anyhow::Result<()> {
    use armada::db::Repo;

    let test = common::Test::new().await;

    let latest: BlockWithTxs = get_file("etc/805543-block.json").await?;
    let latest_number = *latest.block_header.block_number.as_ref() as u64;
    let latest_hash =
        NumAsHex::try_new(latest.block_header.block_hash.0.as_ref())?;

    *test.ctx.seq.latest().await = Some(latest.clone());

    test.ctx.db.blocks.put(latest_hash.as_ref(), latest).await?;

    *test.ctx.eth.state().await = Some(armada::eth::State {
        state_block_number: 1,
        state_root: NumAsHex::try_new("0x2")?,
        state_block_hash: NumAsHex::try_new("0x3")?,
    });

    let ctx = test.ctx.clone();
    let src = Source::new(ctx);
    let d = Duration::from_millis(300);
    src.add("seq", sync::poll_seq, d).await;
    src.add("eth", sync::poll_eth, d * 2).await;
    let mut src = src.run();

    // Line below is useful when debugging event stream
    //while let Some(e) = src.get().await { println!("{e:?}"); }

    match src.get().await.expect("one") {
        Event::Head(number, hash) => {
            assert_eq!(number, latest_number);
            assert_eq!(hash.as_ref(), latest_hash.as_ref());
        }
        other => anyhow::bail!("Unexpected event: {other:?}"),
    }

    match src.get().await.expect("two") {
        Event::Ethereum(armada::eth::State {
            state_root,
            state_block_hash,
            state_block_number,
        }) => {
            assert_eq!(state_block_number, 1);
            assert_eq!(state_root.as_ref(), "0x2");
            assert_eq!(state_block_hash.as_ref(), "0x3");
        }
        other => anyhow::bail!("Unexpected event: {other:?}"),
    }

    Ok(())
}

async fn get_file<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
    let json = tokio::fs::read_to_string(path).await?;
    let val: T = serde_json::from_str(&json)?;
    Ok(val)
}

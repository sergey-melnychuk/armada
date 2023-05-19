use std::time::Duration;

use armada::{api::gen::NumAsHex, sync::{self, Event, Source}};

mod common;

#[tokio::test]
async fn test_sync_events() -> anyhow::Result<()> {
    let mut test = common::Test::new().await;

    test.ctx_mut().seq.set_test_call_response(101).await;

    test.ctx_mut()
        .eth
        .set_state(armada::eth::State {
            state_block_number: 1,
            state_root: NumAsHex::try_new("0x2")?,
            state_block_hash: NumAsHex::try_new("0x3")?,
        })
        .await;

    let ctx = test.ctx().clone();
    let src = Source::new(ctx);
    let d = Duration::from_millis(300);
    src.add("seq", sync::poll_seq, d).await;
    src.add("eth", sync::poll_eth, d * 2).await;
    let mut src = src.run();

    let one = src.get().await.expect("one");
    assert!(matches!(one, Event::TestSeq(101)), "{:?}", one);

    let two = src.get().await.expect("two");
    assert!(matches!(two, Event::Ethereum(_)), "{:?}", two);

    Ok(())
}

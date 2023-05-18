use std::time::Duration;

use armada::sync::{self, Event, Source};

mod common;

#[tokio::test]
async fn test_sync_events() -> anyhow::Result<()> {
    let mut test = common::Test::new().await;

    test.ctx_mut().seq.set_test_call_response(101).await;
    test.ctx_mut().eth.set_test_call_response(202).await;

    let ctx = test.ctx().clone();
    let src = Source::new(ctx);
    src.add("seq", sync::poll_seq, Duration::from_millis(300))
        .await;
    src.add("eth", sync::poll_eth, Duration::from_millis(600))
        .await;
    let mut src = src.run();

    let one = src.get().await.expect("one");
    assert!(matches!(one, Event::TestSeq(101)), "{:?}", one);

    let two = src.get().await.expect("two");
    assert!(matches!(two, Event::TestEth(202)), "{:?}", two);

    Ok(())
}

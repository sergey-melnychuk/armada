## Ideas

* Concurrent loading of historic blocks. All blocks below "low watermark" are extremely unlikely to get reorganized, so the can be downloaded simultaneously by block number.

* Publishing indices snapshot along with data (e.g. to AWS S3). Index files for a range of blocks can then be merged into single consistent. Or "snapshot index" can be published, e.g. at block 200k for blocks 0..200k inclusive.

* Async RPC trait and async DB impl. Before everything is async, the blocking DB code must be run with `tokio::task::spawn_blocking`.

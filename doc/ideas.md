## Ideas

* Concurrent loading of historic blocks. All blocks below "low watermark" are extremely unlikely to get reorganized, so the can be downloaded simultaneously by block number.

* Publishing indices snapshot along with data (e.g. to AWS S3). Index files for a range of blocks can then be merged into single consistent. Or "snapshot index" can be published, e.g. each consecutive 10k blocks.

* Async RPC trait and async DB impl (branch: `db/async`). Before everything is async, the blocking DB code must be run with `tokio::task::spawn_blocking`.

* Index blocks hash => number (not number => hash). Store as `block/{number}.json.gzip`. Store also `block/{number}.hash` for fast number => hash lookups. This makes pubishing block-range index snapshots way easier. This also makes organizing large number of files into subdirectories easier (e.g. block/123456789.bin => block/123000000/456000/789/123456789.bin makes sure there is max 1000 files per sub-directory).

* Add `~/.armada/lock.pid` and locking machinery on startup to ensure only single instance of armanda is running on the host. Check how exclusive locking on `.yak` files can be enforced and implement it in `yakvdb`.

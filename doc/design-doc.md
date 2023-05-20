# [DRAFT] Design Doc: Armada - Starknet full node

## INTRO

The goal if this document is to describe architecture and implementation of full starknet node as well as trade-offs analized and decisions made along the way, while keeping the [incidental complexity](https://dev.to/alexbunardzic/software-complexity-essential-accidental-and-incidental-3i4d) overhead as low as possible, interfaces clean with [separation of concerns](https://en.wikipedia.org/wiki/Separation_of_concerns), high cohesion and loose coupling between submodules (ideally making implementations replacebe in a seamless manner), and test coverage simple yet reasonable.

## STORAGE

Storage requirements and trade-offs are dictated by the RPC API and respective query patterns. The nature of blockchain data is discrete (comes in blocks), immutable (reorgs are still possible), effectively time-series based, and append-only. Data is never removed at a meaningful scale (amount of data overwritten on reorg could be neglected compared to the full ledger). Append-only nature allows storing most of the data "at rest" in a highly-available setup (likely sharded/replicated), while keeping in mind constant growth of a full dataset. At some point, storing all the data on each node locally (in the embedded database) would not be convenient, so either sharding + replication strategy is necessary, or it can be abstracted away using existing storage solutions (AWS S3). As long as data is immutable and append only, it makes sense to cache "hot" chunks locally to improve latency distribution.

### Metadata

It absolutely makes sense to include metadata into the storage enginge (if running locally: the directory on the filesystem) containing details about specific chain the data belongs to (mainnet, testnet, integration, etc) - it can be considered as a "static metadata" (as it is not expected to change during the lifetime of the data). The "dynamic metadata" such as synced blocks range, current L1/L2 head block and maybe some configuration properties is a good thing to have as well, and it might come in standalone key-value store (in practice a specific file).

### Implementation

Each entity has natural primary key, being it a hash, block number or an address. Hash (or address) is typically 32-bytes long, whereas integer number is typically `u64` (8-bytes). With natural primary key and distinct chunks of data (blocks), it makes sense to consider KV-based storage. To start with, the local filesystem fits the use case perfectly (it can be scaled out to any KV-based data store, with AWS S3 being most popular one). The data format is JSON - text-based and very compression-friendly (for an average block, up to 10x compression should be easily achieved).

### Entities

- BLOCK
  - source: feeder_gateway
  - lookup: by hash, by number, by tag ('pending'/'latest')
- STATE (state update)
  - source: feeder_gateway
- TX (transaction)
  - source: BLOCK
  - lookup: by hash, by block hash/number/tag + index
  - produces: receipt, events
- EVENT
  - source: TX
  - lookup: filter (key?)
- CLASS
  - source: STATE
  - lookup: by hash
- CONTRACT
  - lookup: address
- ACCOUNT
  - source: (TX of type DEPLOY_ACCOUNT?)
  - lookup: address
- NONCE
  - source: STATE
  - lookup: address

### Buckets

- /BLOCK/0x{hash}.json.gzip
- /STATE/0x{hash}.json.gzip
- /CLASS/0x{hash}.json.gzip

### Indices

- BLOCK number => BLOCK hash
  - source: BLOCK
  - 8 bytes => 32 bytes
  - entry: 40 bytes
- TX hash => (BLOCK hash, TX index)
  - source: BLOCK
  - 32 bytes => (32 bytes + 8 bytes)
  - entry: 72 bytes
- (CONTRACT address, [N * KEY], BLOCK number) => [M * DATA] (EVENTS)
  - source: BLOCK
  - (32 bytes, (N = 8 bytes) * 32 bytes, 8 bytes) => (M = 8 bytes) * 32 bytes
  - entry: (N + M) * (32 + 16) + 40 bytes
    - N=3 M=3: 328 bytes
    - N=10 M=10: 1000 bytes
- (CONTRACT address, BLOCK number) => NONCE
  - source: STATE
  - (32 bytes + 8 bytes) => 32 bytes (really 8 should be enough?)
  - entry: 72 bytes (or 48 bytes)
- (CONTRACT address, KEY, BLOCK number) => VALUE (STORAGE)
  - source: STATE
  - (32 bytes, 32 bytes, 8 bytes) => 32 bytes
  - entry: 104 bytes
- (CONTRACT address, BLOCK number) => N * TX hash (ACCOUNT/CONTRACT TXs)
  - source: STATE
  - (32 bytes + 8 bytes) => (N = 8 bytes) * 32 bytes
  - entry: (N + 1) * (32 + 8) + 8 bytes
    - N=3: 168 bytes
    - N=10: 448 bytes
- (CONTRACT address, BLOCK number) => CLASS hash
  - source: CLASS
  - (32 bytes + 8 bytes) => 32 bytes
  - entry: 72 bytes

### Capacity

Example:
- BLOCK
  - 50kb gzipped JSON each
  - 100 transactions
    - 10 events each
      - 1 key + 4 values each
  - 100 storage diffs
    - 10 key-value pairs each
  - 100 nonces
  - 100 contracts total (deployed/declared/replaced/etc)
- STATE
  - 50kb gzipped JSON each
- CLASS
  - 50kb gzipped JSON each

For B+tree with P=4kb page size as storage and for entry of size X bytes, fitting into a block T = P / X times, thus requiring total number of N entries to have B = N / T + 1 leaf blocks (and thus extra B nodes on top), the formula will look like this:

```
P=4096, N=1000, X=?

T = P / X
B = N / T + 1
bytes = 2 * B * P

bytes = 2 * (N / (P / X) + 1) * P

bytes(X) = 2 * (1000 / (4096 / X) + 1) * 4096
```

<!--- TODO: reformat into a table? -->

N=1000:
- data: 1000 * (3 * 50 kb) = 150 Mb
- indices:
  - block: bytes(40) = 88 Kb
  - tx: bytes(72) = 150 Kb
  - events: bytes(568) = 115 Kb
  - nonce: bytes(72) = 150 Kb
  - storage: bytes(104) = 213 Kb
  - account: bytes(448) = 884 Kb
  - class: bytes(72) = 150 Kb
  - TOTAL: 1750 Kb (~1.4% overhead)
- TOTAL: 152 Mb

N=1M:
- data: 1M * (3 * 50 kb) = 150 Gb
- indices:
  - block: bytes(40) = 78 Mb
  - tx: bytes(72) = 139 Mb
  - events: bytes(568) = 1085 Mb
  - nonce: bytes(72) = 139 Mb
  - storage: bytes(104) = 200 Mb
  - account: bytes(448) = 856 Mb
  - class: bytes(72) = 139 Mb
  - TOTAL: 2626 Mb (~1.7% overhead)
- TOTA: 153 Gb

N=10M:
- data: 10M * (3 * 50 kb) = ~1500 Gb
- indices:
  - block: bytes(40) = 764 Mb
  - tx: bytes(72) = 1375 Mb
  - events: bytes(568) = 10835 Mb
  - nonce: bytes(72) = 1375 Mb
  - storage: bytes(104) = 1985 Mb
  - account: bytes(448) = 8546 Mb
  - class: bytes(72) = 1375 Mb
  - TOTAL: 27 Gb (~1.8% overhead)
- TOTA: ~1527 Gb

N=100M:
- data: 100M * (3 * 50 kb) = ~15 Tb
- indices:
  - block: bytes(40) = 7631 Mb
  - tx: bytes(72) = 13734 Mb
  - events: bytes(568) = 108339 Mb
  - nonce: bytes(72) = 13734 Mb
  - storage: bytes(104) = 19838 Mb
  - account: bytes(448) = 85451 Mb
  - class: bytes(72) = 13734 Mb
  - TOTAL: ~258 Gb (~1.6% overhead)
- TOTA: ~15.3 Tb

### Security

The block binary data stored on block-per-file basics allows "source" node to sign the binary (likely a gzipped JSON) with node's key. This way the block data can be verified by any other node that knows the signer's public key. The un-trusted chunks (e.g. with invalid signatures) can be easily detected and "fixed" by separate reconciliation process or by node on per-request basics.

### Notes

Storing 1M+ files in a single directory locally might get tricky, some relevant reading below:
- [unix.stackexchange](https://unix.stackexchange.com/questions/411091/millions-of-small-text-files-in-a-folder)
- [superuser](https://superuser.com/questions/1733104/millions-of-files-in-a-single-directory)
- [serverfault](https://serverfault.com/questions/95444/storing-a-million-images-in-the-filesystem)
- [medium](https://medium.com/@hartator/benchmark-deep-directory-structure-vs-flat-directory-structure-to-store-millions-of-files-on-ext4-cac1000ca28)

## SYNCING

Even though intuitive way to sync blocks is to start from genesis and and continue until the head block is reached, I believe it makes way more sense to fetch L2 head (or L1 head) and start syncing backwards. This way the most recent nonces and storage data are available as soon as they are reached, and historycal data is updated along the way. It makes sense to assume that most queries (say, 90%+) fetch the most recent data, and historical queries ratio is significantly lower.

If syncing needs to be started from some intermediate state, it can actually proceed from both ends simultaniously: (1) pull current head and continue pulling backwards until the matching saved block is found (NOTE: this way any network reorg is handled automatically, without disrupting the syncing flow!); or (2) keep track of the "lowest" sycned block, and continue from there until genesis block is reached.

```
  GENESIS               LO                    HI                    HEAD
 /                     /                     /                     /
0->->->---------------*---------------------*--------------->->->-H...
|                     |                     |                     |
|< < <             <<<|                     |< < <             <<<|
|                     |                     |                     |
|                     |#####################|                     |
        syncing        <---synced blocks--->        syncing
```

### Reorg "Unwinding"

Even though the suggested "top-down" syncing always converges to the longest (and confirment on L1/L2) chain, in case of reorg, the data (transactions, state updates) from rejected blocks need to be un-indexed (removed from indices key by key) the same way it was indexed in the first place. The cost of having lean indices is a necessity to keep them consistent manually.

In practice this means that during sync an extra check needs to be performed for "fresh" block to verify that there is no block at given index registered before. If there is - such "stale" block needs to be marked as rejected and all transactions & state updates un-indexed.

## NETWORK

### Distributed Setup

TODO: Describe how multiple nodes can work together (master-slave, hash-ring, etc)

### P2P

TODO: Describe how peer-to-peer data propagation might work.

## OTHER

#### Example data

```
curl "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0x6f9ca7fb180e21856fe724436c8ac93059732a7d100533242fd6e380cf034f9" | jq > etc/tx.json

curl "https://alpha4.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x3131fa018d520a037686ce3efddeab8f28895662f019ca3ca18a626650f7d1e" | jq > etc/class.json

curl "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=805543" | jq > etc/805543-state-update.json

curl "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=805543" | jq > etc/805543.json

curl -H 'Content-Type: application/json' -d '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["latest",true],"id":42}' https://eth.llamarpc.com | jq > etc/ethereum-latest.json

$ curl https://alpha4.starknet.io/feeder_gateway/get_contract_addresses
{"GpsStatementVerifier": "0x8f97970aC5a9aa8D130d35146F5b59c4aef57963", "Starknet": "0xde29d060D45901Fb19ED6C6e959EB22d8626708e"}
```

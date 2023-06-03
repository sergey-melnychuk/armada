# [DRAFT] Design Doc: Armada - Starknet full node

* [Problems](/doc/problems.md)
* [Ideas](/doc/ideas.md)

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

- /BLOCK
  - 0x{hash}.json.gzip
  - block.yak (block number to block hash)
- /TX
  - index.yak (tx hash to block hash + tx index)
- /EVENT
  - event.yak (contract addr, event key, block number to event data)
- /STATE
  - 0x{hash}.json.gzip
  - nonce.yak (contract addr, block number to nonce value)
  - index.yak (contract addr, key, block number to state value)
- /CLASS
  - 0x{hash}.json.gzip
  - class.yak (contract addr, block number to class hash)

### Indices

- BLOCK number => BLOCK hash
  - source: BLOCK
  - 8 bytes => 32 bytes
  - entry: 40 bytes
- TX hash => (BLOCK hash, TX index)
  - source: BLOCK
  - 32 bytes => (32 bytes + 8 bytes)
  - entry: 72 bytes
- (CONTRACT address, [each KEY], BLOCK number) => TX index (EVENTS)
  - (lookup event data from the specific TX at the specific block)
  - source: BLOCK
  - N * (32 bytes, 32 bytes, 8 bytes) => 8 bytes
  - entry: 32 + 32 + 8 + 8 bytes = 80 bytes
- (CONTRACT address, BLOCK number) => NONCE
  - source: STATE
  - (32 bytes + 8 bytes) => 32 bytes (really 8 should be enough?)
  - entry: 72 bytes (or 48 bytes)
- (CONTRACT address, KEY, BLOCK number) => VALUE (STATE)
  - source: STATE
  - (32 bytes, 32 bytes, 8 bytes) => 32 bytes
  - entry: 104 bytes
- (CONTRACT address, BLOCK number) => CLASS hash
  - source: CLASS
  - (32 bytes + 8 bytes) => 32 bytes
  - entry: 72 bytes

### Account (TODO: suggest efficient indexing strategy)

Iterating through EVENT, NONCE, STATE and CLASS hashes (range `[(addr,0,0)..(addr,[KEY::MAX,]BLOCK::MAX))`) is enough to pull all data for a given address (account?), but perhabs applying more effient indexing makes sense for a use-case "pull all the data for a given account", especially for accounts with a lot of events.

Using a "block marker" can point to the blocks with account events, then just pulling such blocks and extracting the related events might be a reasonable trade-off between space and time complexity.

- (CONTRACT address, BLOCK number) => () (ACCOUNT)
  - source: BLOCK, STATE
  - (32 bytes + 8 bytes) => 0 bytes
  - entry: 40 bytes

### Capacity

Example:
- BLOCK
  - 50kb gzipped JSON each
  - 100 transactions
    - having 20 addresses involved
    - 10 events each
      - 1 key + 4 values each (80 bytes each)
  - 100 storage diffs
    - 10 key-value pairs each (104 bytes ech)
  - 100 nonces
  - 100 contracts total (deployed/declared/replaced/etc)
- STATE
  - 50kb gzipped JSON each
- CLASS
  - 50kb gzipped JSON each

For B+tree with P=4kb page size as storage and for entry of size X bytes, fitting into a block T = P / X times, thus requiring total number of N entries to have B = N / T + 1 leaf blocks (and thus extra B nodes on top), the formula for minimum possible size of the index will look like below. Taking into account storage overhead (on average the storage engine keeps each page half-full to avoid frequent split/merge of pages), the real size of index can be a factor of 2.

```
P=4096, N=1000, X=?

T = P / X
B = N / T + 1
bytes = 2 * B * P

bytes = 2 * (N / (P / X) + 1) * P

bytes(X) = 2 * (1000 / (4096 / X) + 1) * 4096
```

```python
xs = [40, 72, 216 * 10 * 100, 72, 104 * 10 * 100, 176 * 20, 72]

def kb(n, x):
        return 2 * (n / (4096 / x) + 1) * 4096 / 1024 + 1

def mb(n, x):
        return 2 * (n / (4096 / x) + 1) * 4096 / 1024 / 1024 + 1

n = 1000
ys = [kb(n, x) for x in xs]

from functools import reduce
reduce(lambda a, b: a + b, ys)
```

<!--- TODO: reformat into a table? -->

N=1000:
- data: 1000 * (3 * 50 kb) = 150 Mb
- indices:
  - block: bytes(40) = 88 Kb
  - tx: bytes(72) = 150 Kb
  - events: bytes(80 x 1000) = ~150 Mb
  - nonce: bytes(72) = 150 Kb
  - storage: bytes(104 x 1000) = 200 Mb
  - ~~account: bytes(176 x 20) = 6884 Kb~~
  - class: bytes(72) = 150 Kb
- TOTAL: 150 + 360 Mb (data=~30%)

N=10k:
- data: 10k * (3 * 50 kb) = 1500 Mb
- indices:
  - block: bytes(40) = 790 Kb
  - tx: bytes(72) = 1415 Kb
  - events: bytes(80 x 1000) = ~1500 Mb
  - nonce: bytes(72) = 1415 Kb
  - storage: bytes(104 x 1000) = ~2000 Mb
  - ~~account: bytes(176 x 20) = 69 Mb~~
  - class: bytes(72) = 1415 Kb
- TOTAL: 1500 + 3600 Mb (data=~30%)

N=100k:
- data: 100k * (3 * 50 kb) = 15 Gb
- indices:
  - block: bytes(40) = 9 Mb
  - tx: bytes(72) = 15 Mb
  - events: bytes(80 x 1000) = ~15 Gb
  - nonce: bytes(72) = 15 Mb
  - storage: bytes(104 x 1000) = 20 Gb
  - ~~account: bytes(176 x 20) = 7 Mb~~
  - class: bytes(72) = 15 Mb
- TOTAL: 15 + 36 Gb (data=~30%)

N=1M:
- data: 1M * (3 * 50 kb) = 150 Gb
- indices:
  - block: bytes(40) = 77 Mb
  - tx: bytes(72) = 138 Mb
  - events: bytes(80 x 1000) = ~160 Gb
  - nonce: bytes(72) = 138 Mb
  - storage: bytes(104) = ~200 Gb
  - ~~account: bytes(448) = ~7 Mb~~
  - class: bytes(72) = 138 Mb
- TOTA: 150 + 368 Gb (data=~30%)

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
curl "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0x6f9ca7fb180e21856fe724436c8ac93059732a7d100533242fd6e380cf034f9"

curl "https://alpha4.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x3131fa018d520a037686ce3efddeab8f28895662f019ca3ca18a626650f7d1e"

curl "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=805543"

curl "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=805543"

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_state_update?blockNumber=24978"

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockNumber=24978"

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockNumber=12304" | jq | grep status
>  "status": "ACCEPTED_ON_L1",

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockHash=0x7cebd154f03c5f838999351e2a7f5f1346ea161d355155d424e7b4efda52ccd" | jq | grep status
>  "status": "ABORTED",

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockHash=0x1bd1f64828cf2aff0023881344e63f982494b220d5d27057994864680a7f946" | jq | grep number

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockHash=0x47c3637b57c2b079b93c61539950c17e868a28f46cdef28f88521067f21e943"

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_state_update?blockHash=0x47c3637b57c2b079b93c61539950c17e868a28f46cdef28f88521067f21e943"

curl "https://alpha-mainnet.starknet.io/feeder_gateway/get_state_update?blockHash=0x61e7ef6a2a3cc281742de97c4d4a5a21925357aac6a18833c1452c4531787ef"

curl -H 'Content-Type: application/json' -d '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["latest",true],"id":42}' https://eth.llamarpc.com

curl https://alpha4.starknet.io/feeder_gateway/get_contract_addresses
{"Starknet": "0xde29d060D45901Fb19ED6C6e959EB22d8626708e", "GpsStatementVerifier": "0x8f97970aC5a9aa8D130d35146F5b59c4aef57963"}

curl https://alpha-mainnet.starknet.io/feeder_gateway/get_contract_addresses
{"Starknet": "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4", "GpsStatementVerifier": "0x47312450B3Ac8b5b8e247a6bB6d523e7605bDb60"}
```

#### Example queries

```
curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getBlockWithTxHashes","params":[{"block_number":35130}],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getBlockWithTxHashes","params":[{"block_hash":"0x64185eba772257a97d104f9ef14a50f9a6122d04f27f9f2b406474a999c9b68"}],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getStateUpdate","params":[{"block_hash":"0x64185eba772257a97d104f9ef14a50f9a6122d04f27f9f2b406474a999c9b68"}],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getTransactionByBlockIdAndIndex","params":[{"block_hash":"0x64185eba772257a97d104f9ef14a50f9a6122d04f27f9f2b406474a999c9b68"}, 0],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getTransactionByHash","params":["0x6e0d2d6578de1d328a6a87f6db04680dfe8cac69f1f97d26290635396b37b4a"],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getBlockTransactionCount","params":[{"block_hash":"0x64185eba772257a97d104f9ef14a50f9a6122d04f27f9f2b406474a999c9b68"}],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getStorageAt","params":["0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7","0x04778a33a3c9dcad587c2f328738d089421ec50b3f7b9054218072d19228aac",{"block_number":24978}],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getStorageAt","params":["0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7","0x077eb8e45a2f882311243ea41d07afead6a5eff3b9f7e6a4e1850e38dcfe773e",{"block_number":24978}],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getStorageAt","params":["0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7","0x077eb8e45a2f882311243ea41d07afead6a5eff3b9f7e6a4e1850e38dcfe773e","latest"],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getStorageAt","params":["0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3","0x07c115c5843940b647cee0ed0705a8d3f93948b4fd62545afadb32d51578e81",{"block_hash":"0x707e9838fdf9d09f6c23890957db42af0ba3ff3d1b19ad43f04d0e1798e53bb"}],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getNonce","params":[{"block_number":24978},"0x599e583fcaef9dfe541376ce7453990d35610209565708ded8c9718ec8ee884"],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getNonce","params":[{"block_number":24978},"0x692d5328ece7fcd8e8a6a9e9efad5ee2c1e5cdb4af6f6b8e6827347c2df0254"],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getNonce","params":["latest","0x692d5328ece7fcd8e8a6a9e9efad5ee2c1e5cdb4af6f6b8e6827347c2df0254"],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getTransactionReceipt","params":["0x542f051013de7fa072e8bab3dda43a2376d7f273bd3a111d91a2d5fe4f05875"],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_getEvents","params":{"filter":{"address":"0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8","from_block":{"block_number":24958},"to_block":{"block_number":24998},"keys":[["0x134692b230b9e1ffa39098904722134159652b09c5bc41d88d6698779d228ff"]],"chunk_size":100}},"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_syncing","params":[],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_blockNumber","params":[],"id":1}' http://127.0.0.1:9000/rpc/v0.3

curl -H 'Content-type: application/json' -d '{"jsonrpc":"2.0","method":"starknet_blockHashAndNumber","params":[],"id":1}' http://127.0.0.1:9000/rpc/v0.3
```

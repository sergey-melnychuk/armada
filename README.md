# Armada

Armada is a WIP PoC impl of [Starknet](https://www.starknet.io/en) full node.

And a "sibling" of [Pathfinder](https://github.com/eqlabs/pathfinder).

[Design Doc](/doc/design-doc.md)

[Results](/doc/results.md)

### Run

`ARMADA_INFURA_TOKEN=${INFURA_TOKEN} bin/run ${HOME}/Temp/armada integration --metrics`

### Status

- [x] Sequencer client
- [x] Ethereum client
- [x] Sync
  - [x] event producers
    - [x] parent block
    - [x] pending block
    - [x] latest block
    - [x] ethereum state
  - [x] event handlers
    - [x] save block (+index)
    - [x] index transactions
    - [x] events
    - [x] reorg
      - restore chain
      - keep the data
    - [x] state update
      - [x] dto mapping
      - [x] nonce index
      - [x] store index
    - [x] classes
    - [ ] ~~accounts~~
  - [x] sync testkit
- [x] Storage
  - [x] local
  - [x] gzip
  - [ ] async?
  - [ ] remote? (AWS S3)
- [x] Indices
  - [x] local ([yakvdb](https://github.com/sergey-melnychuk/yakvdb))
  - [ ] remote? (AWS DynamoDB)
- [x] Testing
  - [x] basic RPC test
  - [x] basic sync test
  - [x] mock-free testkit
  - [ ] make seq & eth tests hermetic ([httpmock](https://docs.rs/httpmock/latest/httpmock/))
- [x] JSON-RPC API with [iamgroot](https://github.com/sergey-melnychuk/iamgroot)
- [x] JSON-RPC API methods impl:
  - [x] `starknet_getBlockWithTxHashes`
  - [x] `starknet_getBlockWithTxs`
  - [x] `starknet_getStateUpdate`
  - [x] `starknet_getStorageAt`
  - [x] `starknet_getTransactionByHash`
  - [x] `starknet_getTransactionByBlockIdAndIndex`
  - [x] `starknet_getTransactionReceipt`
  - [x] `starknet_getClass`
  - [x] `starknet_getClassHashAt`
  - [x] `starknet_getClassAt`
  - [x] `starknet_getBlockTransactionCount`
  - [ ] ~~`starknet_call`~~ (needs SDK)
  - [ ] ~~`starknet_estimateFee`~~ (needs SDK)
  - [x] `starknet_blockNumber`
  - [x] `starknet_blockHashAndNumber`
  - [x] `starknet_chainId`
  - [ ] ~~`starknet_pendingTransactions`~~ (pending block is ignored)
  - [x] `starknet_syncing`
  - [x] `starknet_getEvents`
  - [x] `starknet_getNonce`
  - [ ] ~~`starknet_addInvokeTransaction`~~ (proxy call)
  - [ ] ~~`starknet_addDeclareTransaction`~~ (proxy call)
  - [ ] ~~`starknet_addDeployAccountTransaction`~~ (proxy call)
  - [ ] ~~`starknet_traceTransaction`~~ (needs SDK)
  - [ ] ~~`starknet_simulateTransaction`~~ (needs SDK)
  - [ ] ~~`starknet_traceBlockTransactions`~~ (needs SDK)

### Relevant Links

- [starknet-rs](https://github.com/xJonathanLEI/starknet-rs)
- [starknet_in_rust](https://github.com/lambdaclass/starknet_in_rust)
- [kakarot-rpc](https://github.com/kkrt-labs/kakarot-rpc)
- [katana](https://github.com/dojoengine/katana)
- [madara](https://github.com/keep-starknet-strange/madara)

### Misc

Count LoC: `find src tests -type f -name "*.rs" | xargs grep . | wc -l`

### *

WIP = Work In Progress

PoC = Proof of Concept

LoC = Lines of Code

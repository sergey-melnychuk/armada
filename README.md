# Armada

Armada is a WIP PoC impl of [Starknet](https://www.starknet.io/en) full node.

And a "sibling" of [Pathfinder](https://github.com/eqlabs/pathfinder).

[Design Doc](/doc/design-doc.md)

### Status

- [x] Sequencer client
- [x] Ethereum client
- [ ] Sync
  - [x] event producers
    - [x] parent block
    - [x] pending block
    - [x] latest block
    - [x] ethereum state
  - [x] event handlers
    - [x] save block (+index)
    - [x] index transactions
    - [ ] events
    - [ ] reorg (purge block)
    - [ ] state update
      - [x] dto mapping
      - [x] nonce index
      - [ ] store index
    - [ ] classes
    - [ ] accounts (?)
  - [x] sync testkit
- [ ] Storage
  - [x] local
  - [ ] gzip
  - [ ] async?
  - [ ] remote (AWS S3)
- [x] Indices
  - [x] local ([yakvdb](https://github.com/sergey-melnychuk/yakvdb))
  - [ ] remote (AWS DynamoDB)
  - [ ] snapshot (for a range of blocks)
- [ ] Testing
  - [ ] make seq & eth tests hermetic ([httpmock](https://docs.rs/httpmock/latest/httpmock/))
- [x] Single shared context
  - effectively a manual "dependency injection"
- [x] JSON-RPC API with [iamgroot](https://github.com/sergey-melnychuk/iamgroot)
- [ ] JSON-RPC API methods impl:
  - [x] `starknet_getBlockWithTxHashes`
  - [x] `starknet_getBlockWithTxs`
  - [x] `starknet_getStateUpdate`
  - [ ] `starknet_getStorageAt`
  - [x] `starknet_getTransactionByHash`
  - [x] `starknet_getTransactionByBlockIdAndIndex`
  - [ ] `starknet_getTransactionReceipt`
  - [ ] `starknet_getClass`
  - [ ] `starknet_getClassHashAt`
  - [ ] `starknet_getClassAt`
  - [ ] `starknet_getBlockTransactionCount`
  - [ ] `starknet_call`
  - [ ] `starknet_estimateFee`
  - [ ] `starknet_blockNumber`
  - [ ] `starknet_blockHashAndNumber`
  - [ ] `starknet_chainId`
  - [ ] `starknet_pendingTransactions`
  - [ ] `starknet_syncing`
  - [ ] `starknet_getEvents`
  - [ ] `starknet_getNonce`
  - [ ] `starknet_addInvokeTransaction`
  - [ ] `starknet_addDeclareTransaction`
  - [ ] `starknet_addDeployAccountTransaction`
  - [ ] ~~`starknet_traceTransaction`~~
  - [ ] ~~`starknet_simulateTransaction`~~
  - [ ] ~~`starknet_traceBlockTransactions`~~

### Relevant Links

- [starknet-rs](https://github.com/xJonathanLEI/starknet-rs)
- [starknet_in_rust](https://github.com/lambdaclass/starknet_in_rust)
- [kakarot-rpc](https://github.com/kkrt-labs/kakarot-rpc)
- [katana](https://github.com/dojoengine/katana)
- [madara](https://github.com/keep-starknet-strange/madara)

### Misc

Count LoC: `find src tests -type f -name "*.rs" | xargs grep . | wc -l`

# armada

WIP PoC impl of [Starknet](https://www.starknet.io/en) full node and a "sibling" of [Pathfinder](https://github.com/eqlabs/pathfinder).

[Design Doc](/doc/design-doc.md)

### Status

- [x] Sequencer client
- [x] Ethereum client
- [ ] Sync
  - [x] event producers
    - [x] "next" block
    - [x] pending block
    - [x] latest block
    - [x] ethereum state
  - [x] event handlers
    - [x] save block
    - [x] reorg (purge block)
    - [ ] state update
    - [ ] classes
    - [ ] accounts
  - [x] sync testkit
- [ ] Storage
  - [x] local
  - [ ] gzip
  - [ ] async?
  - [ ] remote (AWS S3)
- [x] Indices
  - (see the Design Doc for more details)
  - [x] local ([yakvdb](https://github.com/sergey-melnychuk/yakvdb))
  - [ ] remote (AWS DynamoDB)
- [x] Single shared context
- [x] JSON-RPC API with [iamgroot](https://github.com/sergey-melnychuk/iamgroot)
- [ ] JSON-RPC API methods impl:
  - [x] `starknet_getBlockWithTxHashes`
  - [x] `starknet_getBlockWithTxs`
  - [ ] `starknet_getStateUpdate`
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

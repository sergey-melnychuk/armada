# armada
Armada is a sibling of [Pathfinder](https://github.com/eqlabs/pathfinder).

WIP PoC impl of [Starknet](https://www.starknet.io/en) full node.

#### PLAN
- [ ] Sequencer client
- [ ] Event producers
- [ ] Event consumers
- [ ] Ethereum client
- [ ] Storage (durable + in-memory)
- [ ] Single shared context
- [ ] RPC methods impl:
  - [ ] `starknet_getBlockWithTxHashes`
  - [ ] `starknet_getBlockWithTxs`
  - [ ] `starknet_getStateUpdate`
  - [ ] `starknet_getStorageAt`
  - [ ] `starknet_getTransactionByHash`
  - [ ] `starknet_getTransactionByBlockIdAndIndex`
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
  - [ ] `starknet_traceTransaction`
  - [ ] `starknet_simulateTransaction`
  - [ ] `starknet_traceBlockTransactions`

#### DONE
- [x] JSON-RPC API with [iamgroot](https://github.com/sergey-melnychuk/iamgroot)
- [x] Basic test utils

#### Misc
- [xJonathanLEI/starknet-rs](https://github.com/xJonathanLEI/starknet-rs)
- [lambdaclass/starknet_in_rust](https://github.com/lambdaclass/starknet_in_rust)
- [kkrt-labs/kakarot-rpc](https://github.com/kkrt-labs/kakarot-rpc)

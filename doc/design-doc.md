# [DRAFT] Design Doc: Armada - Starknet full node

## INTRO

The goal if this document is to describe architecture and implementation of full starknet node as well as trade-offs analized and decisions made along the way, while keeping the [incidental complexity](https://dev.to/alexbunardzic/software-complexity-essential-accidental-and-incidental-3i4d) overhead as low as possible, interfaces clean with [separation of concerns](https://en.wikipedia.org/wiki/Separation_of_concerns), high cohesion and loose coupling between submodules (ideally making implementations replacebe in a seamless manner), and test coverage simple yet reasonable.

## USE-CASES

TODO: Describe target use-cases

## STORAGE

Storage requirements and trade-offs are dictated by the RPC API and respective query patterns. The nature of blockchain data is discrete (comes in blocks), immutable (reorgs are still possible), effectively time-series based, and append-only. Data is never removed at a meaningful scale (amount of data overwritten on reorg could be neglected compared to the full ledger). Append-only nature allows storing most of the data "at rest" in a highly-available setup (likely sharded/replicated), while keeping in mind constant growth of a full dataset. At some point, storing all the data on each node locally (in the embedded database) would not be convenient, so either sharding + replication strategy is necessary, or it can be abstracted away using existing storage solutions (AWS S3). As long as data is immutable and append only, it makes sense to cache "hot" chunks locally to improve latency distribution.

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

### Implementation

Each entity has natural primary key, being it a hash, block number or an address. Hash (or address) is typically 32-bytes long, whereas integer number is typically `u64` (8-bytes). With natural primary key and distinct chunks of data (blocks), it makes sense to consider KV-based storage. To start with, the local filesystem fits the use case perfectly (it can be scaled out to any KV-based data store, with AWS S3 being most popular one). The data format is JSON - text-based and very compression-friendly (for an average block, up to 10x compression should be easily achieved).

### Capacity

TODO: Estimate necessary capacity for 1M blocks (data storage + index storage)

## NETWORK

### Distributed Setup

TODO: Describe how multiple nodes can work together (master-slave, etc)

### P2P

TODO: Describe how peer-to-peer data propagation might work.

## OTHER

#### Example data

```
curl "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=805543" | jq > etc/805543-state-update.json
curl "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=805543" | jq > etc/805543.json
```

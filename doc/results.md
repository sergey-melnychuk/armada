## Results

During just 2 weeks multiple design suggestions were implemented and validated:
- storage (filesystem-like)
- indexing (lean KV-indices)
- event-driven sync (producer-consumer)
- single shared application context
- testkit (based on shared context)

### Testnet full sync summary

Total time: ~7 days (throttled to 100 blocks per minute)

```
$ du -sh ~/armada/testnet
56G	~/armada/testnet
```

```
$ tree -h ~/armada/testnet/ | grep -v json
[4.0K]  ~/armada/testnet/
├── [ 95M]  block
│   ├── [4.6G]  event.yak
│   └── [108M]  index.yak
├── [2.3M]  class
│   └── [239M]  index.yak
├── [ 96M]  state
│   ├── [ 32G]  index.yak
│   └── [876M]  nonce.yak
└── [4.0K]  tx
    └── [2.8G]  index.yak

4 directories, 1651840 files
```

```
$ ls -l ~/armada/testnet/state | grep json | wc -l
815954
$ ls -l ~/armada/testnet/block | grep json | wc -l
815954
$ ls -l ~/armada/testnet/class | grep json | wc -l
19927
```

### Mainnet full sync summary

Total time: 27h 25m

```
$ du -sh ~/armada/mainnet
25G	~/armada/mainnet
```

```
$ tree -h ~/armada/mainnet/ | grep -v json
[4.0K]  ~/armada/mainnet/
├── [8.0M]  block
│   ├── [2.4G]  event.yak
│   └── [9.1M]  index.yak
├── [444K]  class
│   └── [209M]  index.yak
├── [8.0M]  state
│   ├── [ 12G]  index.yak
│   └── [1.5G]  nonce.yak
└── [4.0K]  tx
    └── [1.9G]  index.yak

4 directories, 140880 files
```

```
$ ls -l ~/armada/mainnet/state | grep json | wc -l
68553
$ ls -l ~/armada/mainnet/block | grep json | wc -l
68553
$ ls -l ~/armada/mainnet/class | grep json | wc -l
3769
```

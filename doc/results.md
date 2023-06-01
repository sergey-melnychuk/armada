## Results

During just 2 weeks multiple design suggestions were implemented and validated:
- storage (filesystem-like)
- indexing (lean KV-indices)
- event-driven sync (producer-consumer)
- single shared application context
- testkit (based on shared context)

### Mainnet full sync summary

Total time: 27h 25m

```
$ du -sh ~/Temp/armada/mainnet
25G	~/Temp/armada/mainnet
```

```
$ tree -h ~/Temp/armada/mainnet/ | grep -v json
[4.0K]  ~/Temp/armada/mainnet/
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
$ ls -l ~/Temp/armada/mainnet/state | grep json | wc -l
68553
$ ls -l ~/Temp/armada/mainnet/block | grep json | wc -l
68553
$ ls -l ~/Temp/armada/mainnet/class | grep json | wc -l
3769
```

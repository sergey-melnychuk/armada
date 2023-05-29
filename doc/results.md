## Armada experiment: Results

During just 2 weeks multiple design suggestions were implemented and validated:
- storage (filesystem-like)
- indexing (lean KV-indices)
- event-driven sync (producer-consumer)
- single shared application context
- testkit (based on shared context)

### Mainnet full sync summary

Total time: ~36h

```
24G   ~/armada/mainnet/

6.9G  ~/armada/mainnet/block
15G   ~/armada/mainnet/state
298M  ~/armada/mainnet/class

[4.0K]  ~/armada/mainnet/
├── [7.8M]  block
│   ├── [2.3G]  event.yak
│   └── [8.8M]  index.yak
├── [468K]  class
│   └── [206M]  index.yak
├── [7.8M]  state
│   ├── [ 11G]  index.yak
│   └── [1.4G]  nonce.yak
└── [4.0K]  tx
    └── [1.9G]  index.yak
```

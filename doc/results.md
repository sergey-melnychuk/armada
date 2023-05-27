## Armada experiment: Results

I consider Armada experiment a success. During just 2 weeks multiple design suggestions are implemented and validated:
- storage (filesystem-like)
- indexing (lean KV-indices)
- event-driven sync (producer-consumer breakdown)

### Mainnet full sync

NOTE!: Class data is missing in this sync.

Total time: **27h 15m**

Total size: **24G**

```
$ du -sh ~/Temp/armada/mainnet/
24G	~/Temp/armada/mainnet/

$ ls -l ~/Temp/armada/mainnet/state/ | grep json | wc -l
65589

$ ls -l ~/Temp/armada/mainnet/block/ | grep json | wc -l
65589

$ tree -h ~/Temp/armada/mainnet/ | grep -v json
[4.0K]  ~/Temp/armada/mainnet/
├── [7.7M]  block
│   ├── [2.3G]  event.yak
│   └── [8.7M]  index.yak
├── [7.7M]  state
│   ├── [ 11G]  index.yak
│   └── [1.4G]  nonce.yak
└── [4.0K]  tx
    └── [1.8G]  index.yak

3 directories, 131183 files
```

```
$ cat run.log | grep 0x47c3637b57c2b079b93c61539950c17e868a28f46cdef28f88521067f21e943
2023-05-27T13:55:13.513455Z  INFO armada::sync: Block saved number=0 hash="0x47c3637b57c2b079b93c61539950c17e868a28f46cdef28f88521067f21e943"
2023-05-27T13:55:17.433566Z  INFO armada::sync: State saved number=0 hash="0x47c3637b57c2b079b93c61539950c17e868a28f46cdef28f88521067f21e943"

$ head -n 10 run.log
2023-05-26T10:41:46.497259Z  INFO armada: Sycned blocks lo=0 hi=0
2023-05-26T10:41:46.497547Z  INFO armada: RPC server listening at=0.0.0.0:9000
2023-05-26T10:41:46.598890Z  INFO armada::sync: uptime seconds=0
2023-05-26T10:41:50.391379Z  INFO armada::sync: L2 head number=64347 hash="0x58dd38cbea34a6e7ade4f514be441e31c3414f0925e752c5b1fe0de0126871b"
2023-05-26T10:41:57.079274Z  INFO armada::sync: Block saved number=64346 hash="0x3d820f18cda345d1289e8bc4d8de71cc6ff5f3ca7cd641ba4f6291131f2ff45"
2023-05-26T10:41:57.778981Z  INFO armada::sync: State saved number=64346 hash="0x3d820f18cda345d1289e8bc4d8de71cc6ff5f3ca7cd641ba4f6291131f2ff45"
2023-05-26T10:41:59.781342Z  INFO armada::sync: L1 head number=63817 
<snip>
```

```
$ cat mainnet-run.log | grep -v INFO
2023-05-26T23:31:49.201557Z  WARN armada::sync: Poll failed name="eth" reason=Failed to parse hex number
2023-05-27T10:13:13.152098Z  WARN armada::sync: Unexpected block status number=12304 hash="0x7cebd154f03c5f838999351e2a7f5f1346ea161d355155d424e7b4efda52ccd" status=Aborted
2023-05-27T10:13:13.989301Z  WARN armada::sync: Unexpected block status number=12302 hash="0x14d51955b90b1d74e9cf22bf3352c6a7d13036203c65da7bee77b9d7a5f6ab7" status=Aborted
2023-05-27T10:13:16.293309Z  WARN armada::sync: Unexpected block status number=12297 hash="0x3dc5e7fd184af0c07d1a7542d93d0ba933dc355502fa1336ab252589c5b5a38" status=Aborted
2023-05-27T10:13:16.721749Z  WARN armada::sync: Unexpected block status number=12296 hash="0x5f28108855e545894b750836148d1e65f200c159ad52230155b74b14156a477" status=Aborted
2023-05-27T10:17:24.947731Z  WARN armada::sync: Poll failed name="eth" reason=Failed to parse hex number
2023-05-27T16:09:37.578164Z  WARN armada::sync: Poll failed name="eth" reason=Failed to parse hex number
```

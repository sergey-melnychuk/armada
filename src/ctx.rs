use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::{runtime::Runtime, sync::Mutex, time::Instant};
use yakvdb::typed::DB;

use crate::{
    api::gen::*,
    cfg::Config,
    db::{AddressAndNumber, AddressWithKeyAndNumber, BlockAndIndex, Repo, Storage},
    eth::EthApi,
    seq::SeqApi,
    util::{get_txn_receipt, map_class, map_state_update, tx_hash, U256, U64},
};

#[derive(Clone, Debug)]
pub struct Head {
    pub block_number: u64,
    pub block_hash: Felt,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            block_number: 0,
            block_hash: Felt::try_new("0x0").unwrap(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Shared {
    pub head: Head,
}

#[allow(clippy::declare_interior_mutable_const)] // clippy, this time just fuck off
static RUNTIME: Lazy<Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

#[derive(Clone)]
pub struct Context<ETH, SEQ> {
    pub since: Instant,
    pub db: Storage,
    pub eth: ETH,
    pub seq: SEQ,
    pub shared: Arc<Mutex<Shared>>,
    pub config: Config,
}

impl<ETH, SEQ> Context<ETH, SEQ>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    pub fn new(eth: ETH, seq: SEQ, shared: Shared, db: Storage, config: Config) -> Self {
        Self {
            since: Instant::now(),
            db,
            eth,
            seq,
            shared: Arc::new(Mutex::new(shared)),
            config,
        }
    }

    pub fn shared(&self) -> Arc<Mutex<Shared>> {
        self.shared.clone()
    }

    fn get_block_number(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<u64, iamgroot::jsonrpc::Error> {
        let block_number = match block_id {
            BlockId::BlockNumber { block_number } => *block_number.as_ref() as u64,
            BlockId::BlockHash { block_hash } => {
                let key = block_hash.0.as_ref();
                let block = self
                    .db
                    .blocks
                    .get(key)
                    .map_err(|e| {
                        iamgroot::jsonrpc::Error::new(
                            -65000,
                            format!("Failed to fetch block '{}': {:?}", key, e),
                        )
                    })?
                    .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;
                *block.block_header.block_number.as_ref() as u64
            }
            BlockId::BlockTag(BlockTag::Latest) => u64::MAX,
            _ => {
                return Err(iamgroot::jsonrpc::Error::new(
                    -1,
                    "'Pending' block is not supported".to_string(),
                ));
            }
        };
        Ok(block_number)
    }
}

impl<ETH, SEQ> crate::api::gen::Rpc for Context<ETH, SEQ>
where
    ETH: EthApi,
    SEQ: SeqApi,
{
    fn getBlockWithTxHashes(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetBlockWithTxHashesResult, iamgroot::jsonrpc::Error> {
        let block = match self.getBlockWithTxs(block_id)? {
            GetBlockWithTxsResult::BlockWithTxs(block) => block,
            _ => {
                return Err(crate::api::gen::error::BLOCK_NOT_FOUND.into());
            }
        };

        let txs = block
            .block_body_with_txs
            .transactions
            .iter()
            .map(|tx| tx_hash(tx).clone())
            .collect::<Vec<_>>();

        Ok(GetBlockWithTxHashesResult::BlockWithTxHashes(
            BlockWithTxHashes {
                block_body_with_tx_hashes: BlockBodyWithTxHashes { transactions: txs },
                block_header: block.block_header.clone(),
                status: block.status,
            },
        ))
    }

    fn getBlockWithTxs(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetBlockWithTxsResult, iamgroot::jsonrpc::Error> {
        let hash = match block_id {
            BlockId::BlockHash { block_hash } => block_hash,
            _ => {
                return Err(crate::api::gen::error::BLOCK_NOT_FOUND.into());
            }
        };

        let key = hash.0.as_ref();
        let mut block = self
            .db
            .blocks
            .get(key)
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    -65000,
                    format!("Failed to fetch block '{}': {:?}", key, e),
                )
            })?
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;
        block.receipts.clear();

        Ok(GetBlockWithTxsResult::BlockWithTxs(block))
    }

    fn getStateUpdate(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetStateUpdateResult, iamgroot::jsonrpc::Error> {
        let hash = match block_id {
            BlockId::BlockHash { block_hash } => block_hash,
            _ => {
                return Err(crate::api::gen::error::BLOCK_NOT_FOUND.into());
            }
        };

        let key = hash.0.as_ref();
        let state = self
            .db
            .states
            .get(key)
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    -65000,
                    format!("Failed to fetch block '{}': {:?}", key, e),
                )
            })?
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        let state_update = map_state_update(state);
        Ok(GetStateUpdateResult::StateUpdate(state_update))
    }

    fn getStorageAt(
        &self,
        contract_address: Address,
        key: StorageKey,
        block_id: BlockId,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        let block_number = match block_id {
            BlockId::BlockNumber { block_number } => *block_number.as_ref() as u64,
            BlockId::BlockHash { block_hash } => {
                let key = block_hash.0.as_ref();
                let block = self
                    .db
                    .blocks
                    .get(key)
                    .map_err(|e| {
                        iamgroot::jsonrpc::Error::new(
                            -65000,
                            format!("Failed to fetch block '{}': {:?}", key, e),
                        )
                    })?
                    .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;
                *block.block_header.block_number.as_ref() as u64
            }
            BlockId::BlockTag(BlockTag::Latest) => u64::MAX,
            _ => {
                return Err(crate::api::gen::error::BLOCK_NOT_FOUND.into());
            }
        };

        let address = U256::from_hex(contract_address.0.as_ref()).map_err(|e| {
            iamgroot::jsonrpc::Error::new(-65000, format!("Failed to read address: '{e}'"))
        })?;
        let storage_key = U256::from_hex(key.as_ref()).map_err(|e| {
            iamgroot::jsonrpc::Error::new(-65000, format!("Failed to read key: '{e}'"))
        })?;
        let number = U64::from_u64(block_number);
        let item = AddressWithKeyAndNumber::from(address, storage_key, number);

        let item = if block_number == u64::MAX {
            RUNTIME
                .block_on(async { self.db.states_index.read().await.below(&item) })
                .ok()
                .flatten()
                .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?
        } else {
            item
        };

        let result = RUNTIME
            .block_on(async {
                self.db
                    .states_index
                    .read()
                    .await
                    .lookup(&item)
                    .map_err(|e| {
                        iamgroot::jsonrpc::Error::new(-65000, format!("Failed to read key: '{e}'"))
                    })
            })?
            .or_else(|| {
                RUNTIME
                    .block_on(async { self.db.states_index.read().await.below(&item) })
                    .ok()
                    .flatten()
                    .and_then(|item| {
                        RUNTIME
                            .block_on(async { self.db.states_index.read().await.lookup(&item) })
                            .ok()
                            .flatten()
                    })
            })
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        let felt = Felt::try_new(&result.into_str())?;
        Ok(felt)
    }

    fn getTransactionByHash(
        &self,
        transaction_hash: TxnHash,
    ) -> std::result::Result<Txn, iamgroot::jsonrpc::Error> {
        let key = U256::from_hex(transaction_hash.0.as_ref()).map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                -65000,
                format!(
                    "Failed to read TX hash '{}': {:?}",
                    transaction_hash.0.as_ref(),
                    e
                ),
            )
        })?;

        let block_and_number: BlockAndIndex = RUNTIME
            .block_on(async { self.db.txs_index.read().await.lookup(&key) })
            .map_err(|_| crate::api::gen::error::BLOCK_NOT_FOUND)?
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        let hash = Felt::try_new(&block_and_number.block().into_str())?;
        self.getTransactionByBlockIdAndIndex(
            BlockId::BlockHash {
                block_hash: BlockHash(hash),
            },
            Index::try_new(block_and_number.index().into_u64() as i64)?,
        )
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        block_id: BlockId,
        index: Index,
    ) -> std::result::Result<Txn, iamgroot::jsonrpc::Error> {
        let index = *index.as_ref() as usize;

        let hash = match block_id {
            BlockId::BlockHash { block_hash } => block_hash,
            _ => {
                return Err(crate::api::gen::error::BLOCK_NOT_FOUND.into());
            }
        };

        let key = hash.0.as_ref();
        let block = self
            .db
            .blocks
            .get(key)
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    -65000,
                    format!("Failed to fetch block '{}': {:?}", key, e),
                )
            })?
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        if let Some(txn) = block.block_body_with_txs.transactions.get(index) {
            Ok(txn.clone())
        } else {
            Err(crate::api::gen::error::BLOCK_NOT_FOUND.into())
        }
    }

    fn getTransactionReceipt(
        &self,
        transaction_hash: TxnHash,
    ) -> std::result::Result<TxnReceipt, iamgroot::jsonrpc::Error> {
        let key = U256::from_hex(transaction_hash.0.as_ref()).map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                -65000,
                format!(
                    "Failed to read TX hash '{}': {:?}",
                    transaction_hash.0.as_ref(),
                    e
                ),
            )
        })?;

        let block_and_index: BlockAndIndex = RUNTIME
            .block_on(async { self.db.txs_index.read().await.lookup(&key) })
            .map_err(|_| crate::api::gen::error::BLOCK_NOT_FOUND)?
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        let block_hash = block_and_index.block();
        let tx_index = block_and_index.index().into_u64() as usize;

        let key = &block_hash.into_str();
        let block: BlockWithTxs = self
            .db
            .blocks
            .get(key)
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    -65000,
                    format!("Failed to fetch block '{}': {:?}", key, e),
                )
            })?
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        let txn_receipt = get_txn_receipt(block, tx_index);
        Ok(txn_receipt)
    }

    fn getClass(
        &self,
        _block_id: BlockId,
        class_hash: Felt,
    ) -> std::result::Result<GetClassResult, iamgroot::jsonrpc::Error> {
        // TODO: Class hash does not relly depend on block. Does it?

        let class = self
            .db
            .classes
            .get(class_hash.as_ref())
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    -65000,
                    format!("Failed to fetch class '{}': {:?}", class_hash.as_ref(), e),
                )
            })?
            .ok_or(crate::api::gen::error::CLASS_HASH_NOT_FOUND)?;

        Ok(map_class(class))
    }

    fn getClassHashAt(
        &self,
        block_id: BlockId,
        contract_address: Address,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        let block_number = self.get_block_number(block_id)?;

        let address = U256::from_hex(contract_address.0.as_ref()).unwrap();
        let number = U64::from_u64(block_number);
        let key = AddressAndNumber::from(address, number);
        let class_hash: U256 = RUNTIME
            .block_on(async { self.db.classes_index.read().await.lookup(&key) })
            .map_err(|_| crate::api::gen::error::CLASS_HASH_NOT_FOUND)?
            .ok_or(crate::api::gen::error::CLASS_HASH_NOT_FOUND)?;

        let felt = Felt::try_new(&class_hash.into_str()).unwrap();
        Ok(felt)
    }

    fn getClassAt(
        &self,
        block_id: BlockId,
        contract_address: Address,
    ) -> std::result::Result<GetClassAtResult, iamgroot::jsonrpc::Error> {
        let class_hash = self.getClassHashAt(block_id.clone(), contract_address)?;
        let class = self.getClass(block_id, class_hash)?;
        Ok(match class {
            GetClassResult::ContractClass(contract_class) => {
                GetClassAtResult::ContractClass(contract_class)
            }
            GetClassResult::DeprecatedContractClass(deprecated_contract_class) => {
                GetClassAtResult::DeprecatedContractClass(deprecated_contract_class)
            }
        })
    }

    fn getBlockTransactionCount(
        &self,
        block_id: BlockId,
    ) -> std::result::Result<GetBlockTransactionCountResult, iamgroot::jsonrpc::Error> {
        let block = match self.getBlockWithTxs(block_id)? {
            GetBlockWithTxsResult::BlockWithTxs(block) => block,
            _ => {
                return Err(crate::api::gen::error::BLOCK_NOT_FOUND.into());
            }
        };

        let n = block.block_body_with_txs.transactions.len() as i64;

        GetBlockTransactionCountResult::try_new(n)
    }

    fn call(
        &self,
        _request: FunctionCall,
        _block_id: BlockId,
    ) -> std::result::Result<CallResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn estimateFee(
        &self,
        _request: Request,
        _block_id: BlockId,
    ) -> std::result::Result<EstimateFeeResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn blockNumber(&self) -> std::result::Result<BlockNumber, iamgroot::jsonrpc::Error> {
        let num = RUNTIME.block_on(async {
            let idx = self.db.blocks_index.read().await;
            let max = idx.max()?;
            Ok(max)
        })
        .map_err(|e: anyhow::Error| {
            iamgroot::jsonrpc::Error::new(
                -1,
                format!("Failed to read block index: {e:?}"),
            )
        })?
        .ok_or(iamgroot::jsonrpc::Error::new(
            -1,
            format!("Failed to read block index"),
        ))?;

        BlockNumber::try_new(num.into_u64() as i64)
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<BlockHashAndNumberResult, iamgroot::jsonrpc::Error> {
        let (num, hash) = RUNTIME.block_on(async {
            let idx = self.db.blocks_index.read().await;
            let max = idx.max()?;
            let max_hash = if let Some(max) = max.as_ref() {
                idx.lookup(max)?
            } else {
                None
            };
            Ok(max.zip(max_hash))
        })
        .map_err(|e: anyhow::Error| {
            iamgroot::jsonrpc::Error::new(
                -1,
                format!("Failed to read block index: {e:?}"),
            )
        })?
        .ok_or(iamgroot::jsonrpc::Error::new(
            -1,
            format!("Failed to read block index"),
        ))?;

        Ok(BlockHashAndNumberResult {
            block_hash: Some(BlockHash(Felt::try_new(&hash.into_str())?)),
            block_number: Some(BlockNumber::try_new(num.into_u64() as i64)?),
        })
    }

    fn chainId(&self) -> std::result::Result<ChainId, iamgroot::jsonrpc::Error> {
        ChainId::try_new("0xCAFEBABE") // TODO: make it right
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<PendingTransactionsResult, iamgroot::jsonrpc::Error> {
        Err(iamgroot::jsonrpc::Error::new(
            -1,
            "'Pending' block is not supported".to_string(),
        ))
    }

    fn syncing(&self) -> std::result::Result<SyncingSyncing, iamgroot::jsonrpc::Error> {
        let (lo, lo_hash, hi, hi_hash) = RUNTIME.block_on(async {
            let idx = self.db.blocks_index.read().await;
            let min = idx.min()?;
            let max = idx.max()?;
            let min_hash = if let Some(min) = min.as_ref() {
                idx.lookup(min)?
            } else {
                None
            };
            let max_hash = if let Some(max) = max.as_ref() {
                idx.lookup(max)?
            } else {
                None
            };
            Ok((min, min_hash, max, max_hash))
        })
        .map_err(|e: anyhow::Error| {
            iamgroot::jsonrpc::Error::new(
                -1,
                format!("Failed to read block index: {e:?}"),
            )
        })?;
        let (lo, lo_hash, hi, hi_hash) = match (lo, lo_hash, hi, hi_hash) {
            (Some(lo), Some(lo_hash), Some(hi), Some(hi_hash)) => (lo, lo_hash, hi, hi_hash),
            _ => return Ok(SyncingSyncing::Boolean(false)),
        };
        let hi_hash = BlockHash(Felt::try_new(&hi_hash.into_str())?);
        let hi_num = NumAsHex::try_new(&hi.into_str())?;
        let lo_hash = BlockHash(Felt::try_new(&lo_hash.into_str())?);
        let lo_num = NumAsHex::try_new(&lo.into_str())?;
        Ok(SyncingSyncing::SyncStatus(SyncStatus {
            current_block_hash: hi_hash.clone(),
            current_block_num: hi_num.clone(),
            highest_block_hash: hi_hash,
            highest_block_num: hi_num,
            starting_block_hash: lo_hash,
            starting_block_num: lo_num,
        }))
    }

    fn getEvents(
        &self,
        filter: Filter,
    ) -> std::result::Result<EventsChunk, iamgroot::jsonrpc::Error> {
        let addr = filter
            .event_filter
            .address
            .map(|addr| U256::from_hex(addr.0.as_ref()).unwrap())
            .ok_or(iamgroot::jsonrpc::Error::new(
                -1,
                "Address is undefined".to_string(),
            ))?;

        let lo = if let Some(from_block) = filter.event_filter.from_block {
            self.get_block_number(from_block)?
        } else {
            0
        };

        let hi = if let Some(to_block) = filter.event_filter.to_block {
            self.get_block_number(to_block)?
        } else {
            u64::MAX
        };

        if hi - lo + 1 > 100 {
            return Err(iamgroot::jsonrpc::Error::new(
                -1,
                format!("Too many blocks: {}", hi - lo + 1),
            ));
        }

        let keys: Vec<U256> = filter
            .event_filter
            .keys
            .map(|vec| vec.into_iter().flatten().collect())
            .map(|vec: Vec<Felt>| {
                vec.into_iter()
                    .map(|felt| U256::from_hex(felt.as_ref()).unwrap())
                    .collect()
            })
            .unwrap_or_default();

        if keys.len() > 100 {
            return Err(iamgroot::jsonrpc::Error::new(
                -1,
                format!("Too many keys: {}", keys.len()),
            ));
        }

        let mut events: Vec<EmittedEvent> = Vec::new();
        for n in lo..=hi {
            let number = U64::from_u64(n);
            for k in &keys {
                let key = AddressWithKeyAndNumber::from(addr.clone(), k.clone(), number.clone());

                let found = RUNTIME.block_on(async {
                    let block = self
                        .db
                        .blocks_index
                        .read()
                        .await
                        .lookup(&number)
                        .map_err(|e| {
                            iamgroot::jsonrpc::Error::new(
                                -1,
                                format!("Failed to fetch block: {e:?}"),
                            )
                        })?
                        .and_then(|hash| self.db.blocks.get(&hash.into_str()).ok().flatten());

                    let tx = self
                        .db
                        .events_index
                        .read()
                        .await
                        .lookup(&key)
                        .map_err(|e| {
                            iamgroot::jsonrpc::Error::new(
                                -1,
                                format!("Failed to lookup event: {e:?}"),
                            )
                        })?
                        .map(|x| x.into_u64() as usize);

                    Ok::<Option<(BlockWithTxs, usize)>, iamgroot::jsonrpc::Error>(block.zip(tx))
                })?;

                found
                    .map(|(block, tx)| {
                        let receipt = &block.receipts[tx];
                        let transaction_hash = receipt.transaction_hash.clone();
                        receipt
                            .events
                            .clone()
                            .into_iter()
                            .filter(|event| event.from_address.0.as_ref() == &addr.into_str())
                            .filter(|event| {
                                event
                                    .event_content
                                    .keys
                                    .iter()
                                    .any(|key| key.as_ref() == &k.into_str())
                            })
                            .map(move |event| EmittedEvent {
                                block_hash: block.block_header.block_hash.clone(),
                                block_number: block.block_header.block_number.clone(),
                                event,
                                transaction_hash: transaction_hash.clone(),
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default()
                    .into_iter()
                    .for_each(|event| events.push(event));
            }
        }

        Ok(EventsChunk {
            continuation_token: None,
            events,
        })
    }

    fn getNonce(
        &self,
        block_id: BlockId,
        contract_address: Address,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        let block_number = match block_id {
            BlockId::BlockNumber { block_number } => *block_number.as_ref() as u64,
            BlockId::BlockHash { block_hash } => {
                let key = block_hash.0.as_ref();
                let block = self
                    .db
                    .blocks
                    .get(key)
                    .map_err(|e| {
                        iamgroot::jsonrpc::Error::new(
                            -65000,
                            format!("Failed to fetch block '{}': {:?}", key, e),
                        )
                    })?
                    .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;
                *block.block_header.block_number.as_ref() as u64
            }
            BlockId::BlockTag(BlockTag::Latest) => u64::MAX,
            _ => {
                return Err(crate::api::gen::error::BLOCK_NOT_FOUND.into());
            }
        };

        let address = U256::from_hex(contract_address.0.as_ref()).map_err(|e| {
            iamgroot::jsonrpc::Error::new(-65000, format!("Failed to read address: '{e}'"))
        })?;
        let number = U64::from_u64(block_number);
        let item = AddressAndNumber::from(address, number);

        let item = if block_number == u64::MAX {
            RUNTIME
                .block_on(async { self.db.nonces_index.read().await.below(&item) })
                .ok()
                .flatten()
                .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?
        } else {
            item
        };

        let result = RUNTIME
            .block_on(async {
                self.db
                    .nonces_index
                    .read()
                    .await
                    .lookup(&item)
                    .map_err(|e| {
                        iamgroot::jsonrpc::Error::new(
                            -65000,
                            format!("Failed to read nonce: '{e}'"),
                        )
                    })
            })?
            .or_else(|| {
                RUNTIME
                    .block_on(async { self.db.nonces_index.read().await.below(&item) })
                    .ok()
                    .flatten()
                    .and_then(|item| {
                        RUNTIME
                            .block_on(async { self.db.nonces_index.read().await.lookup(&item) })
                            .ok()
                            .flatten()
                    })
            })
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        let felt = Felt::try_new(&result.into_str())?;
        Ok(felt)
    }

    fn addInvokeTransaction(
        &self,
        _invoke_transaction: BroadcastedInvokeTxn,
    ) -> std::result::Result<AddInvokeTransactionResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn addDeclareTransaction(
        &self,
        _declare_transaction: BroadcastedDeclareTxn,
    ) -> std::result::Result<AddDeclareTransactionResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn addDeployAccountTransaction(
        &self,
        _deploy_account_transaction: BroadcastedDeployAccountTxn,
    ) -> std::result::Result<AddDeployAccountTransactionResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn traceTransaction(
        &self,
        _transaction_hash: TxnHash,
    ) -> std::result::Result<TransactionTrace, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn simulateTransaction(
        &self,
        _block_id: BlockId,
        _transaction: Transaction,
        _simulation_flags: SimulationFlags,
    ) -> std::result::Result<SimulateTransactionSimulatedTransactions, iamgroot::jsonrpc::Error>
    {
        not_implemented()
    }

    fn traceBlockTransactions(
        &self,
        _block_hash: BlockHash,
    ) -> std::result::Result<TraceBlockTransactionsTraces, iamgroot::jsonrpc::Error> {
        not_implemented()
    }
}

fn not_implemented<T>() -> std::result::Result<T, iamgroot::jsonrpc::Error> {
    Err(iamgroot::jsonrpc::Error::new(
        -64001,
        "Not Implemented".to_string(),
    ))
}

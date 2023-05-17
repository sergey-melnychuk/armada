use crate::{
    api::gen::*,
    db::{Repo, Storage},
};

#[derive(Clone)]
pub struct Context {
    storage: Storage,
    // TODO: shared
    // TODO: etc
}

impl Context {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl crate::api::gen::Rpc for Context {
    fn getBlockWithTxHashes(
        &self,
        _block_id: BlockId,
    ) -> std::result::Result<GetBlockWithTxHashesResult, iamgroot::jsonrpc::Error> {
        Ok(GetBlockWithTxHashesResult::BlockWithTxHashes(
            BlockWithTxHashes {
                block_body_with_tx_hashes: BlockBodyWithTxHashes {
                    transactions: vec![
                        Felt::try_new("0x1")?,
                        Felt::try_new("0x2")?,
                        Felt::try_new("0x3")?,
                    ],
                },
                block_header: BlockHeader {
                    block_hash: BlockHash(Felt::try_new("0x0")?),
                    block_number: BlockNumber::try_new(0)?,
                    new_root: Felt::try_new("0x0")?,
                    parent_hash: BlockHash(Felt::try_new("0x0")?),
                    sequencer_address: Felt::try_new("0x0")?,
                    timestamp: 42,
                },
                status: BlockStatus::Pending,
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
        let block = self
            .storage
            .blocks()
            .get(key)
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    -65000,
                    format!("Failed to fetch block '{}': {:?}", key, e),
                )
            })?
            .ok_or(crate::api::gen::error::BLOCK_NOT_FOUND)?;

        Ok(GetBlockWithTxsResult::BlockWithTxs(block))
    }

    fn getStateUpdate(
        &self,
        _block_id: BlockId,
    ) -> std::result::Result<GetStateUpdateResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getStorageAt(
        &self,
        _contract_address: Address,
        _key: StorageKey,
        _block_id: BlockId,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getTransactionByHash(
        &self,
        _transaction_hash: TxnHash,
    ) -> std::result::Result<Txn, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        _block_id: BlockId,
        _index: Index,
    ) -> std::result::Result<Txn, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getTransactionReceipt(
        &self,
        _transaction_hash: TxnHash,
    ) -> std::result::Result<TxnReceipt, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getClass(
        &self,
        _block_id: BlockId,
        _class_hash: Felt,
    ) -> std::result::Result<GetClassResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getClassHashAt(
        &self,
        _block_id: BlockId,
        _contract_address: Address,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getClassAt(
        &self,
        _block_id: BlockId,
        _contract_address: Address,
    ) -> std::result::Result<GetClassAtResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getBlockTransactionCount(
        &self,
        _block_id: BlockId,
    ) -> std::result::Result<GetBlockTransactionCountResult, iamgroot::jsonrpc::Error> {
        not_implemented()
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
        not_implemented()
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<BlockHashAndNumberResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn chainId(&self) -> std::result::Result<ChainId, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<PendingTransactionsResult, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn syncing(&self) -> std::result::Result<SyncingSyncing, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getEvents(
        &self,
        _filter: Filter,
    ) -> std::result::Result<EventsChunk, iamgroot::jsonrpc::Error> {
        not_implemented()
    }

    fn getNonce(
        &self,
        _block_id: BlockId,
        _contract_address: Address,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        not_implemented()
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

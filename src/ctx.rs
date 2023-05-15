use crate::rpc::gen::*;

#[derive(Clone)]
pub struct Context {
    // TODO: storage
    // TODO: shared
    // TODO: etc
}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::rpc::gen::Rpc for Context {
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
        _block_id: BlockId,
    ) -> std::result::Result<GetBlockWithTxsResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getStateUpdate(
        &self,
        _block_id: BlockId,
    ) -> std::result::Result<GetStateUpdateResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getStorageAt(
        &self,
        _contract_address: Address,
        _key: StorageKey,
        _block_id: BlockId,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getTransactionByHash(
        &self,
        _transaction_hash: TxnHash,
    ) -> std::result::Result<Txn, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getTransactionByBlockIdAndIndex(
        &self,
        _block_id: BlockId,
        _index: Index,
    ) -> std::result::Result<Txn, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getTransactionReceipt(
        &self,
        _transaction_hash: TxnHash,
    ) -> std::result::Result<TxnReceipt, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getClass(
        &self,
        _block_id: BlockId,
        _class_hash: Felt,
    ) -> std::result::Result<GetClassResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getClassHashAt(
        &self,
        _block_id: BlockId,
        _contract_address: Address,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getClassAt(
        &self,
        _block_id: BlockId,
        _contract_address: Address,
    ) -> std::result::Result<GetClassAtResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getBlockTransactionCount(
        &self,
        _block_id: BlockId,
    ) -> std::result::Result<GetBlockTransactionCountResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn call(
        &self,
        _request: FunctionCall,
        _block_id: BlockId,
    ) -> std::result::Result<CallResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn estimateFee(
        &self,
        _request: Request,
        _block_id: BlockId,
    ) -> std::result::Result<EstimateFeeResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn blockNumber(&self) -> std::result::Result<BlockNumber, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn blockHashAndNumber(
        &self,
    ) -> std::result::Result<BlockHashAndNumberResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn chainId(&self) -> std::result::Result<ChainId, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn pendingTransactions(
        &self,
    ) -> std::result::Result<PendingTransactionsResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn syncing(&self) -> std::result::Result<SyncingSyncing, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getEvents(
        &self,
        _filter: Filter,
    ) -> std::result::Result<EventsChunk, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn getNonce(
        &self,
        _block_id: BlockId,
        _contract_address: Address,
    ) -> std::result::Result<Felt, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn addInvokeTransaction(
        &self,
        _invoke_transaction: BroadcastedInvokeTxn,
    ) -> std::result::Result<AddInvokeTransactionResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn addDeclareTransaction(
        &self,
        _declare_transaction: BroadcastedDeclareTxn,
    ) -> std::result::Result<AddDeclareTransactionResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn addDeployAccountTransaction(
        &self,
        _deploy_account_transaction: BroadcastedDeployAccountTxn,
    ) -> std::result::Result<AddDeployAccountTransactionResult, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn traceTransaction(
        &self,
        _transaction_hash: TxnHash,
    ) -> std::result::Result<TransactionTrace, iamgroot::jsonrpc::Error> {
        todo!()
    }

    fn simulateTransaction(
        &self,
        _block_id: BlockId,
        _transaction: Transaction,
        _simulation_flags: SimulationFlags,
    ) -> std::result::Result<SimulateTransactionSimulatedTransactions, iamgroot::jsonrpc::Error>
    {
        todo!()
    }

    fn traceBlockTransactions(
        &self,
        _block_hash: BlockHash,
    ) -> std::result::Result<TraceBlockTransactionsTraces, iamgroot::jsonrpc::Error> {
        todo!()
    }
}

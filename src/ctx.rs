use std::sync::Arc;

use tokio::{sync::Mutex, time::Instant};

use crate::{
    api::gen::*,
    cfg::Config,
    db::{Repo, Storage},
    eth::EthApi,
    seq::SeqApi,
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
    ETH: EthApi + Send + Sync + 'static,
    SEQ: SeqApi + Send + Sync + 'static,
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
}

impl<ETH, SEQ> crate::api::gen::Rpc for Context<ETH, SEQ>
where
    ETH: EthApi + Send + Sync + 'static,
    SEQ: SeqApi + Send + Sync + 'static,
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
            .map(|tx| match tx {
                Txn::DeclareTxn(DeclareTxn::DeclareTxnV1(txn)) => {
                    txn.common_txn_properties.transaction_hash.0.clone()
                }
                Txn::DeclareTxn(DeclareTxn::DeclareTxnV2(txn)) => txn
                    .declare_txn_v1
                    .common_txn_properties
                    .transaction_hash
                    .0
                    .clone(),
                Txn::DeployAccountTxn(txn) => txn.common_txn_properties.transaction_hash.0.clone(),
                Txn::DeployTxn(txn) => txn.transaction_hash.0.clone(),
                Txn::InvokeTxn(txn) => txn.common_txn_properties.transaction_hash.0.clone(),
                Txn::L1HandlerTxn(txn) => txn.transaction_hash.0.clone(),
            })
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

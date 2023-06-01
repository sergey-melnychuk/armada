use std::cell::RefCell;

use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver, Sender},
    task::JoinHandle,
};

use crate::{
    api::gen::{
        Address, BlockHash, BlockStatus, BlockWithTxs, CommonReceiptProperties, ContractClass,
        ContractClassEntryPoint, ContractStorageDiffItem, DeclareTxn, DeclareTxnReceipt,
        DeclareTxnReceiptType, DeclaredClassesItem, DeployAccountTxnReceipt,
        DeployAccountTxnReceiptType, DeployTxnReceipt, DeployTxnReceiptType, DeployedContractItem,
        Felt, GetClassResult, InvokeTxnReceipt, InvokeTxnReceiptType, L1HandlerTxnReceipt,
        L1HandlerTxnReceiptType, NoncesItem, PendingStateUpdate, ReplacedClassesItem,
        SierraEntryPoint, StateDiff, StateUpdate, StorageEntriesItem, Txn, TxnHash, TxnReceipt,
        TxnStatus,
    },
    seq::dto::{self, DeclaredClass, DeployedContract, ReplacedClass},
};

pub mod http {
    pub const HTTP_OK: u16 = 200;
}

#[derive(Clone, Debug, Default)]
pub struct U256(pub [u8; 32]);

impl U256 {
    pub fn from_hex(hex: &str) -> anyhow::Result<Self> {
        let mut slice = [0u8; 32];
        let hex = format!("{:0>64}", hex.strip_prefix("0x").unwrap_or(hex));
        hex::decode_to_slice(hex, &mut slice)?;
        Ok(Self(slice))
    }

    pub fn into_str(&self) -> String {
        let unpadded = hex::encode(self.0)
            .chars()
            .skip_while(|c| c == &'0')
            .collect::<String>();
        format!("0x{}", unpadded)
    }

    pub fn into_str_pad(&self) -> String {
        format!("0x{:0>64}", hex::encode(self.0))
    }
}

impl AsRef<[u8]> for U256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for U256 {
    fn from(value: &'a [u8]) -> Self {
        let mut hex = [0u8; 32];
        hex.copy_from_slice(value);
        Self(hex)
    }
}

#[derive(Clone, Debug, Default)]
pub struct U64(pub [u8; 8]);

impl U64 {
    pub fn from_u64(x: u64) -> Self {
        Self(x.to_be_bytes())
    }

    pub fn into_u64(&self) -> u64 {
        u64::from_be_bytes(self.0)
    }

    pub fn into_str(&self) -> String {
        let unpadded = hex::encode(self.0)
            .chars()
            .skip_while(|c| c == &'0')
            .collect::<String>();
        format!("0x{}", unpadded)
    }
}

impl AsRef<[u8]> for U64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for U64 {
    fn from(value: &'a [u8]) -> Self {
        let mut hex = [0u8; 8];
        hex.copy_from_slice(value);
        Self(hex)
    }
}

pub struct Waiter {
    jh: RefCell<Option<JoinHandle<()>>>,
    tx: RefCell<Option<Sender<()>>>,
}

impl Waiter {
    pub fn new(jh: JoinHandle<()>, tx: Sender<()>) -> Self {
        Self {
            jh: RefCell::new(Some(jh)),
            tx: RefCell::new(Some(tx)),
        }
    }

    pub fn unfold(&mut self) -> Option<(JoinHandle<()>, Sender<()>)> {
        if self.jh.borrow().is_none() || self.tx.borrow().is_none() {
            return None;
        }
        let jh = self.jh.borrow_mut().take().unwrap();
        let tx = self.tx.borrow_mut().take().unwrap();
        Some((jh, tx))
    }

    pub async fn done(&self) {
        if self.jh.borrow().is_none() {
            return;
        }
        let jh = self.jh.borrow_mut().take().unwrap();
        if let Err(e) = jh.await {
            log::error!("Sync task terminated with error: {e}");
        }
    }

    pub fn stop(&self) {
        if self.tx.borrow().is_none() {
            return;
        }
        let tx = self.tx.borrow_mut().take().unwrap();
        if tx.send(()).is_err() {
            log::error!("Sync shutdown attempt failed");
        }
    }
}

pub fn is_open(rx: &mut Receiver<()>) -> bool {
    match rx.try_recv() {
        Ok(_) | Err(TryRecvError::Closed) => false,
        Err(TryRecvError::Empty) => true,
    }
}

pub fn patch_pending_block(mut value: serde_json::Value) -> serde_json::Value {
    let n = value["transactions"]
        .as_array()
        .map(|vec| vec.len())
        .unwrap_or_default();
    for idx in 0..n {
        if value["transactions"][idx]["nonce"].as_str().is_none() {
            value["transactions"][idx]["nonce"] = serde_json::json!("0x0");
        }
    }
    value
}

pub fn patch_block(mut value: serde_json::Value) -> serde_json::Value {
    if value.get("block_hash").is_none() {
        value["block_hash"] = serde_json::json!("0x0");
    }

    if value.get("state_root").is_none() {
        value["state_root"] = serde_json::json!("0x0");
    }

    if value.get("block_number").is_none() {
        value["block_number"] = serde_json::json!(0);
    }

    patch_pending_block(value)
}

pub fn identity<T>(x: T) -> T {
    x
}

pub fn tx_hash(tx: &Txn) -> &Felt {
    match tx {
        Txn::DeclareTxn(DeclareTxn::DeclareTxnV1(txn)) => {
            &txn.common_txn_properties.transaction_hash.0
        }
        Txn::DeclareTxn(DeclareTxn::DeclareTxnV2(txn)) => {
            &txn.declare_txn_v1.common_txn_properties.transaction_hash.0
        }
        Txn::DeployAccountTxn(txn) => &txn.common_txn_properties.transaction_hash.0,
        Txn::DeployTxn(txn) => &txn.transaction_hash.0,
        Txn::InvokeTxn(txn) => &txn.common_txn_properties.transaction_hash.0,
        Txn::L1HandlerTxn(txn) => &txn.transaction_hash.0,
    }
}

pub fn map_state_update(state: dto::StateUpdate) -> StateUpdate {
    StateUpdate {
        block_hash: BlockHash(state.block_hash),
        new_root: state.new_root,
        pending_state_update: PendingStateUpdate {
            old_root: state.old_root,
            state_diff: StateDiff {
                deployed_contracts: state
                    .state_diff
                    .deployed_contracts
                    .into_iter()
                    .map(
                        |DeployedContract {
                             address,
                             class_hash,
                         }| DeployedContractItem {
                            address,
                            class_hash,
                        },
                    )
                    .collect(),
                nonces: state
                    .state_diff
                    .nonces
                    .into_iter()
                    .map(|(addr, nonce)| NoncesItem {
                        contract_address: Some(Address(addr)),
                        nonce: Some(nonce),
                    })
                    .collect(),
                storage_diffs: state
                    .state_diff
                    .storage_diffs
                    .into_iter()
                    .map(|(addr, diff)| ContractStorageDiffItem {
                        address: addr,
                        storage_entries: diff
                            .into_iter()
                            .map(|kv| StorageEntriesItem {
                                key: Some(kv.key),
                                value: Some(kv.value),
                            })
                            .collect(),
                    })
                    .collect(),
                deprecated_declared_classes: state.state_diff.old_declared_contracts,
                replaced_classes: state
                    .state_diff
                    .replaced_classes
                    .into_iter()
                    .map(
                        |ReplacedClass {
                             address,
                             class_hash,
                         }| ReplacedClassesItem {
                            class_hash: Some(class_hash),
                            contract_address: Some(Address(address)),
                        },
                    )
                    .collect(),
                declared_classes: state
                    .state_diff
                    .declared_classes
                    .into_iter()
                    .map(
                        |DeclaredClass {
                             class_hash,
                             compiled_class_hash,
                         }| DeclaredClassesItem {
                            class_hash: Some(class_hash),
                            compiled_class_hash: Some(compiled_class_hash),
                        },
                    )
                    .collect(),
            },
        },
    }
}

pub mod gzip {
    use flate2::read::GzDecoder;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::prelude::*;

    pub fn gzip(input: &str) -> anyhow::Result<Vec<u8>> {
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(input.as_ref())?;
        let bytes = e.finish()?;
        Ok(bytes)
    }

    pub fn ungzip(input: &[u8]) -> anyhow::Result<String> {
        let mut d = GzDecoder::new(input);
        let mut ret = String::new();
        d.read_to_string(&mut ret)?;
        Ok(ret)
    }
}

pub fn get_txn_receipt(block: BlockWithTxs, tx_index: usize) -> TxnReceipt {
    let receipt = block.receipts[tx_index].clone();
    let tx = block.block_body_with_txs.transactions[tx_index].clone();

    let common_receipt_properties = CommonReceiptProperties {
        actual_fee: receipt.actual_fee,
        block_hash: block.block_header.block_hash,
        block_number: block.block_header.block_number,
        events: receipt.events,
        messages_sent: receipt.l2_to_l1_messages,
        status: match block.status {
            BlockStatus::AcceptedOnL1 => TxnStatus::AcceptedOnL1,
            BlockStatus::AcceptedOnL2 => TxnStatus::AcceptedOnL2,
            BlockStatus::Pending => TxnStatus::Pending,
            BlockStatus::Rejected => TxnStatus::Rejected,
            BlockStatus::Aborted => TxnStatus::Aborted,
        },
        transaction_hash: TxnHash(tx_hash(&tx).clone()),
    };

    match tx {
        Txn::DeclareTxn(DeclareTxn::DeclareTxnV1(_)) => {
            TxnReceipt::DeclareTxnReceipt(DeclareTxnReceipt {
                common_receipt_properties,
                r#type: DeclareTxnReceiptType::Declare,
            })
        }
        Txn::DeclareTxn(DeclareTxn::DeclareTxnV2(_)) => {
            TxnReceipt::DeclareTxnReceipt(DeclareTxnReceipt {
                common_receipt_properties,
                r#type: DeclareTxnReceiptType::Declare,
            })
        }
        Txn::DeployTxn(txn) => TxnReceipt::DeployTxnReceipt(DeployTxnReceipt {
            common_receipt_properties,
            contract_address: txn.deploy_txn_properties.contract_address_salt,
            r#type: DeployTxnReceiptType::Deploy,
        }),
        Txn::DeployAccountTxn(txn) => {
            TxnReceipt::DeployAccountTxnReceipt(DeployAccountTxnReceipt {
                common_receipt_properties,
                contract_address: txn.deploy_account_txn_properties.contract_address_salt,
                r#type: DeployAccountTxnReceiptType::DeployAccount,
            })
        }
        Txn::InvokeTxn(_) => TxnReceipt::InvokeTxnReceipt(InvokeTxnReceipt {
            common_receipt_properties,
            r#type: InvokeTxnReceiptType::Invoke,
        }),
        Txn::L1HandlerTxn(_) => TxnReceipt::L1HandlerTxnReceipt(L1HandlerTxnReceipt {
            common_receipt_properties,
            r#type: L1HandlerTxnReceiptType::L1Handler,
        }),
    }
}

pub fn map_class(_class: dto::Class) -> GetClassResult {
    // TODO: implement mapping

    GetClassResult::ContractClass(ContractClass {
        abi: Some("abi".to_string()),
        contract_class_version: "v1".to_string(),
        entry_points_by_type: ContractClassEntryPoint {
            constructor: Some(vec![
                SierraEntryPoint {
                    function_idx: Some(101),
                    selector: Some(Felt::try_new("0x102").unwrap()),
                },
                SierraEntryPoint {
                    function_idx: Some(103),
                    selector: Some(Felt::try_new("0x104").unwrap()),
                },
            ]),
            external: Some(vec![
                SierraEntryPoint {
                    function_idx: Some(201),
                    selector: Some(Felt::try_new("0x202").unwrap()),
                },
                SierraEntryPoint {
                    function_idx: Some(203),
                    selector: Some(Felt::try_new("0x204").unwrap()),
                },
            ]),
            l1_handler: Some(vec![
                SierraEntryPoint {
                    function_idx: Some(301),
                    selector: Some(Felt::try_new("0x302").unwrap()),
                },
                SierraEntryPoint {
                    function_idx: Some(303),
                    selector: Some(Felt::try_new("0x304").unwrap()),
                },
            ]),
        },
        sierra_program: vec![
            Felt::try_new("0x1").unwrap(),
            Felt::try_new("0x2").unwrap(),
            Felt::try_new("0x3").unwrap(),
        ],
    })
}

pub mod is_done {
    use std::{sync::Arc, time::Duration};

    use tokio::sync::RwLock;
    use yakvdb::typed::{Store, DB};

    use super::{U256, U64};

    #[allow(dead_code)]
    pub async fn is_done(index: Arc<RwLock<Store<U64, U256>>>) {
        let delay = 5 * Duration::from_secs(60);
        tokio::spawn(async move {
            loop {
                if let Some(min) = index.read().await.min()? {
                    if min.into_u64() == 0 {
                        tracing::info!("Sync is complete");
                        break;
                    }
                }
                tokio::time::sleep(delay).await;
            }
            Ok::<_, anyhow::Error>(())
        })
        .await
        .map(|_| ())
        .ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u256_from_hex() -> anyhow::Result<()> {
        let hex = "55aaf50c8001b89bcc180c4be977ec519b401ced8366e2e2da78dc5285306d8";
        let num = U256::from_hex(hex)?;
        assert_eq!(num.into_str(), "0x".to_string() + hex);
        assert_eq!(num.into_str_pad(), "0x0".to_string() + hex);
        Ok(())
    }

    #[test]
    fn test_gzip_roundtrip() -> anyhow::Result<()> {
        let message = "The quick brown fox jumps over the lazy dog";
        let bytes = gzip::gzip(message.as_ref())?;
        let restore = gzip::ungzip(&bytes)?;
        assert_eq!(restore, message);
        Ok(())
    }
}

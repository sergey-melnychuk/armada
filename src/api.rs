// vvv GENERATED CODE BELOW vvv
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(clippy::enum_variant_names)]
pub mod gen {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use iamgroot::jsonrpc;

    // object: 'ADDRESS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Address(pub Felt); // name != binding_name

    // object: 'BLOCK_BODY_WITH_TXS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxs {
        pub transactions: Vec<Txn>,
    }

    // object: 'BLOCK_BODY_WITH_TX_HASHES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxHashes {
        pub transactions: Vec<Felt>,
    }

    // object: 'BLOCK_HASH'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHash(pub Felt); // name != binding_name

    // object: 'BLOCK_HEADER'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHeader {
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        #[serde(alias = "state_root")]
        pub new_root: Felt,
        #[serde(alias = "parent_block_hash")]
        pub parent_hash: BlockHash,
        pub sequencer_address: Felt,
        pub timestamp: i64,
    }

    // object: 'BLOCK_ID'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockId {
        BlockHash { block_hash: BlockHash },
        BlockNumber { block_number: BlockNumber },
        BlockTag(BlockTag),
    }

    // object: 'BLOCK_NUMBER'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct BlockNumber(pub i64); // name == binding_name
    #[serde(try_from = "i64")]
    pub struct BlockNumber(i64);

    mod blocknumber {
        use super::jsonrpc;
        use super::BlockNumber;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl BlockNumber {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("BlockNumber value {value} must be > {MIN}"),
                    });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("BlockNumber value {value} must be < {MAX}"),
                    });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for BlockNumber {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for BlockNumber {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    // object: 'BLOCK_STATUS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BlockStatus {
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "ABORTED")]
        Aborted,
    }

    // object: 'BLOCK_TAG'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BlockTag {
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "pending")]
        Pending,
    }

    // object: 'BLOCK_TRANSACTION_TRACE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockTransactionTrace {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trace_root: Option<TransactionTrace>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<Felt>,
    }

    // object: 'BLOCK_WITH_TXS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockWithTxs {
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        pub status: BlockStatus,

        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(rename = "transaction_receipts")]
        pub receipts: Vec<TxnReceiptSummary>,
    }

    // Utility DTO to match response from the gateway
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TxnReceiptSummary {
        pub transaction_index: i64,
        pub transaction_hash: TxnHash,
        pub l2_to_l1_messages: Vec<MsgToL1>,
        pub events: Vec<Event>,
        pub actual_fee: Felt,
    }

    // object: 'BLOCK_WITH_TX_HASHES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockWithTxHashes {
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        pub status: BlockStatus,
    }

    // object: 'BROADCASTED_DECLARE_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BroadcastedDeclareTxn {
        BroadcastedDeclareTxnV1(BroadcastedDeclareTxnV1),
        BroadcastedDeclareTxnV2(BroadcastedDeclareTxnV2),
    }

    // object: 'BROADCASTED_DECLARE_TXN_V1'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV1 {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_class: Option<DeprecatedContractClass>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sender_address: Option<Address>,
    }

    // object: 'BROADCASTED_DECLARE_TXN_V2'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV2 {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compiled_class_hash: Option<Felt>,
        pub contract_class: ContractClass,
        pub sender_address: Address,
        #[serde(rename = "type")]
        pub r#type: BroadcastedDeclareTxnV2Type,
    }

    // object: 'BROADCASTED_DECLARE_TXN_V2_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV2Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    // object: 'BROADCASTED_DEPLOY_ACCOUNT_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeployAccountTxn {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        #[serde(flatten)]
        pub deploy_account_txn_properties: DeployAccountTxnProperties,
    }

    // object: 'BROADCASTED_INVOKE_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedInvokeTxn {
        #[serde(flatten)]
        pub broadcasted_invoke_txn_kind: BroadcastedInvokeTxnKind,
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        #[serde(rename = "type")]
        pub r#type: BroadcastedInvokeTxnType,
    }

    // object: 'BROADCASTED_INVOKE_TXN_KIND'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BroadcastedInvokeTxnKind {
        FunctionCall(FunctionCall),
        InvokeTxnV1(InvokeTxnV1),
    }

    // object: 'BROADCASTED_INVOKE_TXN_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedInvokeTxnType {
        #[serde(alias = "INVOKE_FUNCTION")]
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    // object: 'BROADCASTED_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BroadcastedTxn {
        BroadcastedDeclareTxn(BroadcastedDeclareTxn),
        BroadcastedDeployAccountTxn(BroadcastedDeployAccountTxn),
        BroadcastedInvokeTxn(BroadcastedInvokeTxn),
    }

    // object: 'BROADCASTED_TXN_COMMON_PROPERTIES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedTxnCommonProperties {
        pub max_fee: Felt,
        pub nonce: Felt,
        pub signature: Signature,
        pub version: NumAsHex,
    }

    // object: 'CALL_TYPE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum CallType {
        #[serde(rename = "CALL")]
        Call,
        #[serde(rename = "LIBRARY_CALL")]
        LibraryCall,
    }

    // object: 'CHAIN_ID'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct ChainId(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct ChainId(String);

    mod chainid {
        use super::jsonrpc;
        use super::ChainId;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static CHAINID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[a-fA-F0-9]+$").unwrap());

        impl ChainId {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if CHAINID_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("ChainId value does not match regex: {value}"),
                    })
                }
            }
        }

        impl TryFrom<String> for ChainId {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for ChainId {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'COMMON_RECEIPT_PROPERTIES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CommonReceiptProperties {
        pub actual_fee: Felt,
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        pub events: Vec<Event>,
        pub messages_sent: Vec<MsgToL1>,
        pub status: TxnStatus,
        pub transaction_hash: TxnHash,
    }

    // object: 'COMMON_TXN_PROPERTIES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CommonTxnProperties {
        #[serde(flatten)]
        pub broadcasted_txn_common_properties: BroadcastedTxnCommonProperties,
        pub transaction_hash: TxnHash,
    }

    // object: 'CONTRACT_ABI'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractAbi(pub Vec<ContractAbiEntry>); // name == binding_name

    // object: 'CONTRACT_ABI_ENTRY'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum ContractAbiEntry {
        EventAbiEntry(EventAbiEntry),
        FunctionAbiEntry(FunctionAbiEntry),
        StructAbiEntry(StructAbiEntry),
    }

    // object: 'CONTRACT_CLASS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractClass {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub abi: Option<String>,
        pub contract_class_version: String,
        pub entry_points_by_type: ContractClassEntryPoint,
        pub sierra_program: Vec<Felt>,
    }

    // object: 'CONTRACT_CLASS_ENTRY_POINT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractClassEntryPoint {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "CONSTRUCTOR")]
        pub constructor: Option<Vec<SierraEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "EXTERNAL")]
        pub external: Option<Vec<SierraEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "L1_HANDLER")]
        pub l1_handler: Option<Vec<SierraEntryPoint>>,
    }

    // object: 'CONTRACT_STORAGE_DIFF_ITEM'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractStorageDiffItem {
        pub address: Felt,
        pub storage_entries: Vec<StorageEntriesItem>,
    }

    // object: 'DECLARED_CLASSES_ITEM'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclaredClassesItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub class_hash: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compiled_class_hash: Option<Felt>,
    }

    // object: 'DECLARE_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum DeclareTxn {
        DeclareTxnV1(DeclareTxnV1),
        DeclareTxnV2(DeclareTxnV2),
    }

    // object: 'DECLARE_TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        #[serde(rename = "type")]
        pub r#type: DeclareTxnReceiptType,
    }

    // object: 'DECLARE_TXN_RECEIPT_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnReceiptType {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    // object: 'DECLARE_TXN_TRACE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnTrace {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validate_invocation: Option<FunctionInvocation>,
    }

    // object: 'DECLARE_TXN_V1'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV1 {
        pub class_hash: Felt,
        #[serde(flatten)]
        pub common_txn_properties: CommonTxnProperties,
        pub sender_address: Address,
        #[serde(rename = "type")]
        pub r#type: DeclareTxnV1Type,
    }

    // object: 'DECLARE_TXN_V1_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV1Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    // object: 'DECLARE_TXN_V2'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV2 {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compiled_class_hash: Option<Felt>,
        #[serde(flatten)]
        pub declare_txn_v1: DeclareTxnV1,
    }

    // object: 'DEPLOYED_CONTRACT_ITEM'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployedContractItem {
        pub address: Felt,
        pub class_hash: Felt,
    }

    // object: 'DEPLOY_ACCOUNT_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxn {
        #[serde(flatten)]
        pub common_txn_properties: CommonTxnProperties,
        #[serde(flatten)]
        pub deploy_account_txn_properties: DeployAccountTxnProperties,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_PROPERTIES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnProperties {
        pub class_hash: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub contract_address_salt: Felt,
        #[serde(rename = "type")]
        pub r#type: DeployAccountTxnPropertiesType,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_PROPERTIES_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnPropertiesType {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub contract_address: Felt,
        #[serde(rename = "type")]
        pub r#type: DeployAccountTxnReceiptType,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_RECEIPT_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnReceiptType {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    // object: 'DEPLOY_ACCOUNT_TXN_TRACE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnTrace {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub constructor_invocation: Option<FunctionInvocation>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validate_invocation: Option<FunctionInvocation>,
    }

    // object: 'DEPLOY_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployTxn {
        pub class_hash: Felt,
        #[serde(flatten)]
        pub deploy_txn_properties: DeployTxnProperties,
        pub transaction_hash: TxnHash,
    }

    // object: 'DEPLOY_TXN_PROPERTIES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployTxnProperties {
        pub constructor_calldata: Vec<Felt>,
        pub contract_address_salt: Felt,
        #[serde(rename = "type")]
        pub r#type: DeployTxnPropertiesType,
        pub version: NumAsHex,
    }

    // object: 'DEPLOY_TXN_PROPERTIES_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployTxnPropertiesType {
        #[serde(rename = "DEPLOY")]
        Deploy,
    }

    // object: 'DEPLOY_TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub contract_address: Felt,
        #[serde(rename = "type")]
        pub r#type: DeployTxnReceiptType,
    }

    // object: 'DEPLOY_TXN_RECEIPT_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployTxnReceiptType {
        #[serde(rename = "DEPLOY")]
        Deploy,
    }

    // object: 'DEPRECATED_CAIRO_ENTRY_POINT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedCairoEntryPoint {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub selector: Option<Felt>,
    }

    // object: 'DEPRECATED_CONTRACT_CLASS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedContractClass {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub abi: Option<ContractAbi>,
        pub entry_points_by_type: DeprecatedContractClassEntryPoint,
        pub program: String,
    }

    // object: 'DEPRECATED_CONTRACT_CLASS_ENTRY_POINT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedContractClassEntryPoint {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "CONSTRUCTOR")]
        pub constructor: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "EXTERNAL")]
        pub external: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "L1_HANDLER")]
        pub l1_handler: Option<Vec<DeprecatedCairoEntryPoint>>,
    }

    // object: 'EMITTED_EVENT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EmittedEvent {
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        #[serde(flatten)]
        pub event: Event,
        pub transaction_hash: TxnHash,
    }

    // object: 'ENTRY_POINT_TYPE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum EntryPointType {
        #[serde(rename = "CONSTRUCTOR")]
        Constructor,
        #[serde(rename = "EXTERNAL")]
        External,
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    // object: 'ETH_ADDRESS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct EthAddress(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct EthAddress(String);

    mod ethaddress {
        use super::jsonrpc;
        use super::EthAddress;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static ETHADDRESS_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x[a-fA-F0-9]{40}$").unwrap());

        impl EthAddress {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if ETHADDRESS_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("EthAddress value does not match regex: {value}"),
                    })
                }
            }
        }

        impl TryFrom<String> for EthAddress {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for EthAddress {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'EVENT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Event {
        #[serde(flatten)]
        pub event_content: EventContent,
        pub from_address: Address,
    }

    // object: 'EVENTS_CHUNK'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventsChunk {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continuation_token: Option<String>,
        pub events: Vec<EmittedEvent>,
    }

    // object: 'EVENT_ABI_ENTRY'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventAbiEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keys: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        pub r#type: Option<EventAbiType>,
    }

    // object: 'EVENT_ABI_TYPE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum EventAbiType {
        #[serde(rename = "event")]
        Event,
    }

    // object: 'EVENT_CONTENT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventContent {
        pub data: Vec<Felt>,
        pub keys: Vec<Felt>,
    }

    // object: 'EVENT_FILTER'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventFilter {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_block: Option<BlockId>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keys: Option<Vec<Vec<Felt>>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to_block: Option<BlockId>,
    }

    // object: 'FEE_ESTIMATE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeeEstimate {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gas_consumed: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gas_price: Option<NumAsHex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub overall_fee: Option<NumAsHex>,
    }

    // object: 'FELT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct Felt(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct Felt(String);

    mod felt {
        use super::jsonrpc;
        use super::Felt;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static FELT_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$").unwrap());

        // Workaround to support invalid Felts returned by the gateway.
        fn fix(value: &str) -> String {
            let value = if value.starts_with('0') && !value.starts_with("0x") {
                value.chars().skip_while(|c| c == &'0').collect::<String>()
            } else {
                value.to_string()
            };
            if !value.starts_with("0x") {
                format!("0x{value}")
            } else {
                value
                    .strip_prefix("0x0")
                    .map(|hex| {
                        if hex.is_empty() {
                            value.clone()
                        } else {
                            let hex = hex.chars().skip_while(|c| c == &'0').collect::<String>();
                            format!("0x{hex}")
                        }
                    })
                    .unwrap_or(value)
            }
        }

        impl Felt {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                let value = fix(value);
                if FELT_REGEX.is_match(&value) {
                    Ok(Self(value))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("Felt value does not match regex: {value}"),
                    })
                }
            }
        }

        impl TryFrom<String> for Felt {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Felt {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'FUNCTION_ABI_ENTRY'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionAbiEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inputs: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub outputs: Option<Vec<TypedParameter>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        pub r#type: Option<FunctionAbiType>,
    }

    // object: 'FUNCTION_ABI_TYPE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum FunctionAbiType {
        #[serde(rename = "constructor")]
        Constructor,
        #[serde(rename = "function")]
        Function,
        #[serde(rename = "l1_handler")]
        L1Handler,
    }

    // object: 'FUNCTION_CALL'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionCall {
        pub calldata: Vec<Felt>,
        pub contract_address: Address,
        pub entry_point_selector: Felt,
    }

    // object: 'FUNCTION_INVOCATION'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionInvocation {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub call_type: Option<CallType>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub caller_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub calls: Option<Vec<FunctionInvocation>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entry_point_type: Option<EntryPointType>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub events: Option<Vec<Event>>,
        #[serde(flatten)]
        pub function_call: FunctionCall,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub messages: Option<Vec<MsgToL1>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub result: Option<Vec<Felt>>,
    }

    // object: 'INVOKE_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxn {
        #[serde(flatten)]
        pub common_txn_properties: CommonTxnProperties,
        #[serde(flatten)]
        pub invoke_txn_kind: InvokeTxnKind,
        #[serde(rename = "type")]
        pub r#type: InvokeTxnType,
    }

    // object: 'INVOKE_TXN_KIND'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum InvokeTxnKind {
        FunctionCall(FunctionCall),
        InvokeTxnV1(InvokeTxnV1),
    }

    // object: 'INVOKE_TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        #[serde(rename = "type")]
        pub r#type: InvokeTxnReceiptType,
    }

    // object: 'INVOKE_TXN_RECEIPT_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnReceiptType {
        #[serde(alias = "INVOKE_FUNCTION")]
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    // object: 'INVOKE_TXN_TRACE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnTrace {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub execute_invocation: Option<FunctionInvocation>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub validate_invocation: Option<FunctionInvocation>,
    }

    // object: 'INVOKE_TXN_V0'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV0 {
        pub calldata: Vec<Felt>,
        pub contract_address: Address,
        pub entry_point_selector: Felt,
    }

    // object: 'INVOKE_TXN_V1'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV1 {
        pub calldata: Vec<Felt>,
        pub sender_address: Address,
    }

    // object: 'INVOKE_TXN_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnType {
        #[serde(alias = "INVOKE_FUNCTION")]
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    // object: 'L1_HANDLER_TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxn {
        #[serde(flatten)]
        pub function_call: FunctionCall,
        pub nonce: NumAsHex,
        pub transaction_hash: TxnHash,
        #[serde(rename = "type")]
        pub r#type: L1HandlerTxnType,
        pub version: NumAsHex,
    }

    // object: 'L1_HANDLER_TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        #[serde(rename = "type")]
        pub r#type: L1HandlerTxnReceiptType,
    }

    // object: 'L1_HANDLER_TXN_RECEIPT_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnReceiptType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    // object: 'L1_HANDLER_TXN_TRACE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxnTrace {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub function_invocation: Option<FunctionInvocation>,
    }

    // object: 'L1_HANDLER_TXN_type'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    // object: 'MSG_TO_L1'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MsgToL1 {
        pub payload: Vec<Felt>,
        pub to_address: Felt,
    }

    // object: 'NESTED_CALL'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NestedCall {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub call_type: Option<CallType>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub caller_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub calls: Option<Vec<FunctionInvocation>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entry_point_type: Option<EntryPointType>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub events: Option<Vec<Event>>,
        #[serde(flatten)]
        pub function_call: FunctionCall,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub messages: Option<Vec<MsgToL1>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub result: Option<Vec<Felt>>,
    }

    // object: 'NONCES_ITEM'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NoncesItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_address: Option<Address>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nonce: Option<Felt>,
    }

    // object: 'NUM_AS_HEX'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct NumAsHex(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct NumAsHex(String);

    mod numashex {
        use super::jsonrpc;
        use super::NumAsHex;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static NUMASHEX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^0x[a-fA-F0-9]+$").unwrap());

        impl NumAsHex {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if NUMASHEX_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("NumAsHex value does not match regex: {value}"),
                    })
                }
            }
        }

        impl TryFrom<String> for NumAsHex {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for NumAsHex {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'PENDING_BLOCK_WITH_TXS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxs {
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_hash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sequencer_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i64>,
    }

    // object: 'PENDING_BLOCK_WITH_TX_HASHES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxHashes {
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_hash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sequencer_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i64>,
    }

    // object: 'PENDING_COMMON_RECEIPT_PROPERTIES'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingCommonReceiptProperties {
        pub actual_fee: Felt,
        pub events: Vec<Event>,
        pub messages_sent: Vec<MsgToL1>,
        pub transaction_hash: TxnHash,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        pub r#type: Option<TxnType>,
    }

    // object: 'PENDING_DEPLOY_TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingDeployTxnReceipt {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_address: Option<Felt>,
        #[serde(flatten)]
        pub pending_common_receipt_properties: PendingCommonReceiptProperties,
    }

    // object: 'PENDING_STATE_UPDATE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingStateUpdate {
        pub old_root: Felt,
        pub state_diff: StateDiff,
    }

    // object: 'PENDING_TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum PendingTxnReceipt {
        PendingCommonReceiptProperties(PendingCommonReceiptProperties),
        PendingDeployTxnReceipt(PendingDeployTxnReceipt),
    }

    // object: 'REPLACED_CLASSES_ITEM'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ReplacedClassesItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub class_hash: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_address: Option<Address>,
    }

    // object: 'RESULT_PAGE_REQUEST'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResultPageRequest {
        pub chunk_size: i64,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continuation_token: Option<String>,
    }

    // object: 'SIERRA_ENTRY_POINT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SierraEntryPoint {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub function_idx: Option<i64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub selector: Option<Felt>,
    }

    // object: 'SIGNATURE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Signature(pub Vec<Felt>); // name == binding_name

    // object: 'SIMULATED_TRANSACTION'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SimulatedTransaction {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_estimation: Option<FeeEstimate>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_trace: Option<TransactionTrace>,
    }

    // object: 'SIMULATION_FLAG'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum SimulationFlag {
        #[serde(rename = "SKIP_EXECUTE")]
        SkipExecute,
        #[serde(rename = "SKIP_VALIDATE")]
        SkipValidate,
    }

    // object: 'STATE_DIFF'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StateDiff {
        pub declared_classes: Vec<DeclaredClassesItem>,
        pub deployed_contracts: Vec<DeployedContractItem>,
        pub deprecated_declared_classes: Vec<Felt>,
        pub nonces: Vec<NoncesItem>,
        pub replaced_classes: Vec<ReplacedClassesItem>,
        pub storage_diffs: Vec<ContractStorageDiffItem>,
    }

    // object: 'STATE_UPDATE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StateUpdate {
        pub block_hash: BlockHash,
        pub new_root: Felt,
        #[serde(flatten)]
        pub pending_state_update: PendingStateUpdate,
    }

    // object: 'STORAGE_ENTRIES_ITEM'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StorageEntriesItem {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Felt>,
    }

    // object: 'STORAGE_KEY'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct StorageKey(pub String); // name == binding_name
    #[serde(try_from = "String")]
    pub struct StorageKey(String);

    mod storagekey {
        use super::jsonrpc;
        use super::StorageKey;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static STORAGEKEY_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new("^0x0[0-7]{1}[a-fA-F0-9]{0,62}$").unwrap());

        impl StorageKey {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if STORAGEKEY_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("StorageKey value does not match regex: {value}"),
                    })
                }
            }
        }

        impl TryFrom<String> for StorageKey {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for StorageKey {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    // object: 'STRUCT_ABI_ENTRY'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StructAbiEntry {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub members: Option<Vec<StructMember>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<i64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        pub r#type: Option<StructAbiType>,
    }

    // object: 'STRUCT_ABI_TYPE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum StructAbiType {
        #[serde(rename = "struct")]
        Struct,
    }

    // object: 'STRUCT_MEMBER'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StructMember {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<i64>,
        #[serde(flatten)]
        pub typed_parameter: TypedParameter,
    }

    // object: 'SYNC_STATUS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SyncStatus {
        pub current_block_hash: BlockHash,
        pub current_block_num: NumAsHex,
        pub highest_block_hash: BlockHash,
        pub highest_block_num: NumAsHex,
        pub starting_block_hash: BlockHash,
        pub starting_block_num: NumAsHex,
    }

    // object: 'TRANSACTION_TRACE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionTrace {
        DeclareTxnTrace(DeclareTxnTrace),
        DeployAccountTxnTrace(DeployAccountTxnTrace),
        InvokeTxnTrace(InvokeTxnTrace),
        L1HandlerTxnTrace(L1HandlerTxnTrace),
    }

    // object: 'TXN'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum Txn {
        DeclareTxn(DeclareTxn),
        DeployAccountTxn(DeployAccountTxn),
        DeployTxn(DeployTxn),
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
    }

    // object: 'TXN_HASH'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TxnHash(pub Felt); // name != binding_name

    // object: 'TXN_RECEIPT'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TxnReceipt {
        DeclareTxnReceipt(DeclareTxnReceipt),
        DeployAccountTxnReceipt(DeployAccountTxnReceipt),
        DeployTxnReceipt(DeployTxnReceipt),
        InvokeTxnReceipt(InvokeTxnReceipt),
        L1HandlerTxnReceipt(L1HandlerTxnReceipt),
        PendingTxnReceipt(PendingTxnReceipt),
    }

    // object: 'TXN_STATUS'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnStatus {
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "ABORTED")]
        Aborted,
    }

    // object: 'TXN_TYPE'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnType {
        #[serde(rename = "DECLARE")]
        Declare,
        #[serde(rename = "DEPLOY")]
        Deploy,
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
        #[serde(alias = "INVOKE_FUNCTION")]
        #[serde(rename = "INVOKE")]
        Invoke,
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    // object: 'TYPED_PARAMETER'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TypedParameter {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        pub r#type: Option<String>,
    }

    // object: 'addDeclareTransaction_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddDeclareTransactionResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub class_hash: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<TxnHash>,
    }

    // object: 'addDeployAccountTransaction_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddDeployAccountTransactionResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contract_address: Option<Felt>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<TxnHash>,
    }

    // object: 'addInvokeTransaction_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddInvokeTransactionResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_hash: Option<TxnHash>,
    }

    // object: 'blockHashAndNumber_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHashAndNumberResult {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_hash: Option<BlockHash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_number: Option<BlockNumber>,
    }

    // object: 'blockNumber_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockNumberResult(pub BlockNumber); // name != binding_name

    // object: 'call_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CallResult(pub Vec<Felt>); // name == binding_name

    // object: 'chainId_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ChainIdResult(pub ChainId); // name != binding_name

    // object: 'estimateFee_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EstimateFeeResult(pub Vec<FeeEstimate>); // name == binding_name

    // object: 'filter'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Filter {
        #[serde(flatten)]
        pub event_filter: EventFilter,
        #[serde(flatten)]
        pub result_page_request: ResultPageRequest,
    }

    // object: 'getBlockTransactionCount_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct GetBlockTransactionCountResult(pub i64); // name == binding_name
    #[serde(try_from = "i64")]
    pub struct GetBlockTransactionCountResult(i64);

    mod getblocktransactioncountresult {
        use super::jsonrpc;
        use super::GetBlockTransactionCountResult;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl GetBlockTransactionCountResult {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "GetBlockTransactionCountResult value {value} must be > {MIN}"
                        ),
                    });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "GetBlockTransactionCountResult value {value} must be < {MAX}"
                        ),
                    });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for GetBlockTransactionCountResult {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for GetBlockTransactionCountResult {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    // object: 'getBlockWithTxHashes_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetBlockWithTxHashesResult {
        BlockWithTxHashes(BlockWithTxHashes),
        PendingBlockWithTxHashes(PendingBlockWithTxHashes),
    }

    // object: 'getBlockWithTxs_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetBlockWithTxsResult {
        BlockWithTxs(BlockWithTxs),
        PendingBlockWithTxs(PendingBlockWithTxs),
    }

    // object: 'getClassAt_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetClassAtResult {
        ContractClass(ContractClass),
        DeprecatedContractClass(DeprecatedContractClass),
    }

    // object: 'getClassHashAt_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetClassHashAtResult(pub Felt); // name != binding_name

    // object: 'getClass_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetClassResult {
        ContractClass(ContractClass),
        DeprecatedContractClass(DeprecatedContractClass),
    }

    // object: 'getEvents_events'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetEventsEvents {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub continuation_token: Option<String>,
        pub events: Vec<EmittedEvent>,
    }

    // object: 'getNonce_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetNonceResult(pub Felt); // name != binding_name

    // object: 'getStateUpdate_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetStateUpdateResult {
        PendingStateUpdate(PendingStateUpdate),
        StateUpdate(StateUpdate),
    }

    // object: 'getStorageAt_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetStorageAtResult(pub Felt); // name != binding_name

    // object: 'getTransactionByBlockIdAndIndex_transactionResult'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetTransactionByBlockIdAndIndexTransactionResult {
        DeclareTxn(DeclareTxn),
        DeployAccountTxn(DeployAccountTxn),
        DeployTxn(DeployTxn),
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
    }

    // object: 'getTransactionByHash_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetTransactionByHashResult {
        DeclareTxn(DeclareTxn),
        DeployAccountTxn(DeployAccountTxn),
        DeployTxn(DeployTxn),
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
    }

    // object: 'getTransactionReceipt_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetTransactionReceiptResult {
        DeclareTxnReceipt(DeclareTxnReceipt),
        DeployAccountTxnReceipt(DeployAccountTxnReceipt),
        DeployTxnReceipt(DeployTxnReceipt),
        InvokeTxnReceipt(InvokeTxnReceipt),
        L1HandlerTxnReceipt(L1HandlerTxnReceipt),
        PendingTxnReceipt(PendingTxnReceipt),
    }

    // object: 'index'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    // pub struct Index(pub i64); // name == binding_name
    #[serde(try_from = "i64")]
    pub struct Index(i64);

    mod index {
        use super::jsonrpc;
        use super::Index;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl Index {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("Index value {value} must be > {MIN}"),
                    });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!("Index value {value} must be < {MAX}"),
                    });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for Index {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for Index {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    // object: 'pendingTransactions_result'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingTransactionsResult(pub Vec<Txn>); // name == binding_name

    // object: 'request'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Request(pub Vec<BroadcastedTxn>); // name == binding_name

    // object: 'simulateTransaction_simulated_transactions'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SimulateTransactionSimulatedTransactions(pub Vec<SimulatedTransaction>); // name == binding_name

    // object: 'simulation_flags'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SimulationFlags(pub Vec<SimulationFlag>); // name == binding_name

    // object: 'syncing_syncing'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum SyncingSyncing {
        Boolean(bool),
        SyncStatus(SyncStatus),
    }

    // object: 'traceBlockTransactions_traces'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TraceBlockTransactionsTraces(pub Vec<BlockTransactionTrace>); // name == binding_name

    // object: 'traceTransaction_trace'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TraceTransactionTrace {
        DeclareTxnTrace(DeclareTxnTrace),
        DeployAccountTxnTrace(DeployAccountTxnTrace),
        InvokeTxnTrace(InvokeTxnTrace),
        L1HandlerTxnTrace(L1HandlerTxnTrace),
    }

    // object: 'transaction'
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Transaction(pub Vec<BroadcastedTxn>); // name == binding_name

    pub trait Rpc {
        /// Method: 'starknet_getBlockWithTxHashes'
        /// Summary: Get block information with transaction hashes given the block id
        /// Description:
        ///
        fn getBlockWithTxHashes(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error>;

        /// Method: 'starknet_getBlockWithTxs'
        /// Summary: Get block information with full transactions given the block id
        /// Description:
        ///
        fn getBlockWithTxs(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>;

        /// Method: 'starknet_getStateUpdate'
        /// Summary: Get the information about the result of executing the requested block
        /// Description:
        ///
        fn getStateUpdate(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>;

        /// Method: 'starknet_getStorageAt'
        /// Summary: Get the value of the storage at the given address and key
        /// Description:
        ///
        fn getStorageAt(
            &self,
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Method: 'starknet_getTransactionByHash'
        /// Summary: Get the details and status of a submitted transaction
        /// Description:
        ///
        fn getTransactionByHash(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<Txn, jsonrpc::Error>;

        /// Method: 'starknet_getTransactionByBlockIdAndIndex'
        /// Summary: Get the details of a transaction by a given block id and index
        /// Description: Get the details of the transaction given by the identified block and index in that block. If no transaction is found, null is returned.
        ///
        fn getTransactionByBlockIdAndIndex(
            &self,
            block_id: BlockId,
            index: Index,
        ) -> std::result::Result<Txn, jsonrpc::Error>;

        /// Method: 'starknet_getTransactionReceipt'
        /// Summary: Get the transaction receipt by the transaction hash
        /// Description:
        ///
        fn getTransactionReceipt(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<TxnReceipt, jsonrpc::Error>;

        /// Method: 'starknet_getClass'
        /// Summary: Get the contract class definition in the given block associated with the given hash
        /// Description:
        ///
        fn getClass(
            &self,
            block_id: BlockId,
            class_hash: Felt,
        ) -> std::result::Result<GetClassResult, jsonrpc::Error>;

        /// Method: 'starknet_getClassHashAt'
        /// Summary: Get the contract class hash in the given block for the contract deployed at the given address
        /// Description:
        ///
        fn getClassHashAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Method: 'starknet_getClassAt'
        /// Summary: Get the contract class definition in the given block at the given address
        /// Description:
        ///
        fn getClassAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>;

        /// Method: 'starknet_getBlockTransactionCount'
        /// Summary: Get the number of transactions in a block given a block id
        /// Description: Returns the number of transactions in the designated block.
        ///
        fn getBlockTransactionCount(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockTransactionCountResult, jsonrpc::Error>;

        /// Method: 'starknet_call'
        /// Summary: call a starknet function without creating a StarkNet transaction
        /// Description: Calls a function in a contract and returns the return value.  Using this call will not create a transaction; hence, will not change the state
        ///
        fn call(
            &self,
            request: FunctionCall,
            block_id: BlockId,
        ) -> std::result::Result<CallResult, jsonrpc::Error>;

        /// Method: 'starknet_estimateFee'
        /// Summary: estimate the fee for of StarkNet transactions
        /// Description: estimates the resources required by transactions when applyed on a given state
        ///
        fn estimateFee(
            &self,
            request: Request,
            block_id: BlockId,
        ) -> std::result::Result<EstimateFeeResult, jsonrpc::Error>;

        /// Method: 'starknet_blockNumber'
        /// Summary: Get the most recent accepted block number
        /// Description:
        ///
        fn blockNumber(&self) -> std::result::Result<BlockNumber, jsonrpc::Error>;

        /// Method: 'starknet_blockHashAndNumber'
        /// Summary: Get the most recent accepted block hash and number
        /// Description:
        ///
        fn blockHashAndNumber(
            &self,
        ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>;

        /// Method: 'starknet_chainId'
        /// Summary: Return the currently configured StarkNet chain id
        /// Description:
        ///
        fn chainId(&self) -> std::result::Result<ChainId, jsonrpc::Error>;

        /// Method: 'starknet_pendingTransactions'
        /// Summary: Returns the transactions in the transaction pool, recognized by this sequencer
        /// Description:
        ///
        fn pendingTransactions(
            &self,
        ) -> std::result::Result<PendingTransactionsResult, jsonrpc::Error>;

        /// Method: 'starknet_syncing'
        /// Summary: Returns an object about the sync status, or false if the node is not synching
        /// Description:
        ///
        fn syncing(&self) -> std::result::Result<SyncingSyncing, jsonrpc::Error>;

        /// Method: 'starknet_getEvents'
        /// Summary: Returns all events matching the given filter
        /// Description: Returns all event objects matching the conditions in the provided filter
        ///
        fn getEvents(&self, filter: Filter) -> std::result::Result<EventsChunk, jsonrpc::Error>;

        /// Method: 'starknet_getNonce'
        /// Summary: Get the nonce associated with the given address in the given block
        /// Description:
        ///
        fn getNonce(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Method: 'starknet_addInvokeTransaction'
        /// Summary: Submit a new transaction to be added to the chain
        /// Description:
        ///
        fn addInvokeTransaction(
            &self,
            invoke_transaction: BroadcastedInvokeTxn,
        ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error>;

        /// Method: 'starknet_addDeclareTransaction'
        /// Summary: Submit a new class declaration transaction
        /// Description:
        ///
        fn addDeclareTransaction(
            &self,
            declare_transaction: BroadcastedDeclareTxn,
        ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error>;

        /// Method: 'starknet_addDeployAccountTransaction'
        /// Summary: Submit a new deploy account transaction
        /// Description:
        ///
        fn addDeployAccountTransaction(
            &self,
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        ) -> std::result::Result<AddDeployAccountTransactionResult, jsonrpc::Error>;

        /// Method: 'starknet_traceTransaction'
        /// Summary: For a given executed transaction, return the trace of its execution, including internal calls
        /// Description: Returns the execution trace of the transaction designated by the input hash
        ///
        fn traceTransaction(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<TransactionTrace, jsonrpc::Error>;

        /// Method: 'starknet_simulateTransaction'
        /// Summary: simulate a given transaction on the requested state, and generate the execution trace
        /// Description:
        ///
        fn simulateTransaction(
            &self,
            block_id: BlockId,
            transaction: Transaction,
            simulation_flags: SimulationFlags,
        ) -> std::result::Result<SimulateTransactionSimulatedTransactions, jsonrpc::Error>;

        /// Method: 'starknet_traceBlockTransactions'
        /// Summary: Retrieve traces for all transactions in the given block
        /// Description: Returns the execution traces of all transactions included in the given block
        ///
        fn traceBlockTransactions(
            &self,
            block_hash: BlockHash,
        ) -> std::result::Result<TraceBlockTransactionsTraces, jsonrpc::Error>;
    }

    fn handle_starknet_getBlockWithTxHashes<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxHashes(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getBlockWithTxs<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxs(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getStateUpdate<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getStateUpdate(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getStorageAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, StorageKey, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(contract_address, key, block_id) = args_by_pos;
                ArgByName {
                    contract_address,
                    key,
                    block_id,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            contract_address,
            key,
            block_id,
        } = args;

        match rpc.getStorageAt(contract_address, key, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getTransactionByHash<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction_hash) = args_by_pos;
                ArgByName { transaction_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionByHash(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getTransactionByBlockIdAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Index);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            index: Index,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, index) = args_by_pos;
                ArgByName { block_id, index }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id, index } = args;

        match rpc.getTransactionByBlockIdAndIndex(block_id, index) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getTransactionReceipt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction_hash) = args_by_pos;
                ArgByName { transaction_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionReceipt(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getClass<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Felt);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            class_hash: Felt,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, class_hash) = args_by_pos;
                ArgByName {
                    block_id,
                    class_hash,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            class_hash,
        } = args;

        match rpc.getClass(block_id, class_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getClassHashAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, contract_address) = args_by_pos;
                ArgByName {
                    block_id,
                    contract_address,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getClassHashAt(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getClassAt<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, contract_address) = args_by_pos;
                ArgByName {
                    block_id,
                    contract_address,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getClassAt(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getBlockTransactionCount<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id) = args_by_pos;
                ArgByName { block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockTransactionCount(block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_call<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(FunctionCall, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: FunctionCall,
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(request, block_id) = args_by_pos;
                ArgByName { request, block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { request, block_id } = args;

        match rpc.call(request, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_estimateFee<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Request, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: Request,
            block_id: BlockId,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(request, block_id) = args_by_pos;
                ArgByName { request, block_id }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { request, block_id } = args;

        match rpc.estimateFee(request, block_id) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_blockNumber<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.blockNumber() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_blockHashAndNumber<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.blockHashAndNumber() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_chainId<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.chainId() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_pendingTransactions<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.pendingTransactions() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_syncing<RPC: Rpc>(rpc: &RPC, _params: &Value) -> jsonrpc::Response {
        match rpc.syncing() {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getEvents<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Filter);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filter: Filter,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(filter) = args_by_pos;
                ArgByName { filter }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { filter } = args;

        match rpc.getEvents(filter) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_getNonce<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, contract_address) = args_by_pos;
                ArgByName {
                    block_id,
                    contract_address,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            contract_address,
        } = args;

        match rpc.getNonce(block_id, contract_address) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_addInvokeTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedInvokeTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            invoke_transaction: BroadcastedInvokeTxn,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(invoke_transaction) = args_by_pos;
                ArgByName { invoke_transaction }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { invoke_transaction } = args;

        match rpc.addInvokeTransaction(invoke_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_addDeclareTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeclareTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            declare_transaction: BroadcastedDeclareTxn,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(declare_transaction) = args_by_pos;
                ArgByName {
                    declare_transaction,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            declare_transaction,
        } = args;

        match rpc.addDeclareTransaction(declare_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_addDeployAccountTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeployAccountTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(deploy_account_transaction) = args_by_pos;
                ArgByName {
                    deploy_account_transaction,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            deploy_account_transaction,
        } = args;

        match rpc.addDeployAccountTransaction(deploy_account_transaction) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_traceTransaction<RPC: Rpc>(rpc: &RPC, params: &Value) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(transaction_hash) = args_by_pos;
                ArgByName { transaction_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { transaction_hash } = args;

        match rpc.traceTransaction(transaction_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_simulateTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Transaction, SimulationFlags);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            transaction: Transaction,
            simulation_flags: SimulationFlags,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_id, transaction, simulation_flags) = args_by_pos;
                ArgByName {
                    block_id,
                    transaction,
                    simulation_flags,
                }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName {
            block_id,
            transaction,
            simulation_flags,
        } = args;

        match rpc.simulateTransaction(block_id, transaction, simulation_flags) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    fn handle_starknet_traceBlockTransactions<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_hash: BlockHash,
        }

        let args = serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
            serde_json::from_value::<ArgByPos>(params.clone()).map(|args_by_pos| {
                let ArgByPos(block_hash) = args_by_pos;
                ArgByName { block_hash }
            })
        });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(e) => return jsonrpc::Response::error(-32602, "Invalid params"),
        };

        let ArgByName { block_hash } = args;

        match rpc.traceBlockTransactions(block_hash) {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(-32603, "Internal error"),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    pub fn handle<RPC: Rpc>(rpc: &RPC, req: &jsonrpc::Request) -> jsonrpc::Response {
        let params = &req.params.clone().unwrap_or_default();

        let response = match req.method.as_str() {
            "starknet_getBlockWithTxHashes" => handle_starknet_getBlockWithTxHashes(rpc, params),
            "starknet_getBlockWithTxs" => handle_starknet_getBlockWithTxs(rpc, params),
            "starknet_getStateUpdate" => handle_starknet_getStateUpdate(rpc, params),
            "starknet_getStorageAt" => handle_starknet_getStorageAt(rpc, params),
            "starknet_getTransactionByHash" => handle_starknet_getTransactionByHash(rpc, params),
            "starknet_getTransactionByBlockIdAndIndex" => {
                handle_starknet_getTransactionByBlockIdAndIndex(rpc, params)
            }
            "starknet_getTransactionReceipt" => handle_starknet_getTransactionReceipt(rpc, params),
            "starknet_getClass" => handle_starknet_getClass(rpc, params),
            "starknet_getClassHashAt" => handle_starknet_getClassHashAt(rpc, params),
            "starknet_getClassAt" => handle_starknet_getClassAt(rpc, params),
            "starknet_getBlockTransactionCount" => {
                handle_starknet_getBlockTransactionCount(rpc, params)
            }
            "starknet_call" => handle_starknet_call(rpc, params),
            "starknet_estimateFee" => handle_starknet_estimateFee(rpc, params),
            "starknet_blockNumber" => handle_starknet_blockNumber(rpc, params),
            "starknet_blockHashAndNumber" => handle_starknet_blockHashAndNumber(rpc, params),
            "starknet_chainId" => handle_starknet_chainId(rpc, params),
            "starknet_pendingTransactions" => handle_starknet_pendingTransactions(rpc, params),
            "starknet_syncing" => handle_starknet_syncing(rpc, params),
            "starknet_getEvents" => handle_starknet_getEvents(rpc, params),
            "starknet_getNonce" => handle_starknet_getNonce(rpc, params),
            "starknet_addInvokeTransaction" => handle_starknet_addInvokeTransaction(rpc, params),
            "starknet_addDeclareTransaction" => handle_starknet_addDeclareTransaction(rpc, params),
            "starknet_addDeployAccountTransaction" => {
                handle_starknet_addDeployAccountTransaction(rpc, params)
            }
            "starknet_traceTransaction" => handle_starknet_traceTransaction(rpc, params),
            "starknet_simulateTransaction" => handle_starknet_simulateTransaction(rpc, params),
            "starknet_traceBlockTransactions" => {
                handle_starknet_traceBlockTransactions(rpc, params)
            }
            _ => jsonrpc::Response::error(-32601, "Method not found"),
        };

        return if let Some(id) = req.id.as_ref() {
            response.with_id(id.clone())
        } else {
            response
        };
    }

    pub mod error {
        pub const BLOCK_NOT_FOUND: Error = Error(24, "Block not found");
        pub const CLASS_HASH_NOT_FOUND: Error = Error(28, "Class hash not found");
        pub const CONTRACT_ERROR: Error = Error(40, "Contract error");
        pub const CONTRACT_NOT_FOUND: Error = Error(20, "Contract not found");
        pub const FAILED_TO_RECEIVE_TXN: Error = Error(1, "Failed to write transaction");
        pub const INVALID_BLOCK_HASH: Error = Error(24, "Invalid block hash");
        pub const INVALID_CALL_DATA: Error = Error(22, "Invalid call data");
        pub const INVALID_CONTINUATION_TOKEN: Error =
            Error(33, "The supplied continuation token is invalid or unknown");
        pub const INVALID_CONTRACT_CLASS: Error = Error(50, "Invalid contract class");
        pub const INVALID_MESSAGE_SELECTOR: Error = Error(21, "Invalid message selector");
        pub const INVALID_TXN_HASH: Error = Error(25, "Invalid transaction hash");
        pub const INVALID_TXN_INDEX: Error = Error(27, "Invalid transaction index in a block");
        pub const NO_BLOCKS: Error = Error(32, "There are no blocks");
        pub const NO_TRACE_AVAILABLE: Error = Error(10, "No trace available for transaction");
        pub const PAGE_SIZE_TOO_BIG: Error = Error(31, "Requested page size is too big");
        pub const TOO_MANY_KEYS_IN_FILTER: Error = Error(34, "Too many keys provided in a filter");
        pub const TXN_HASH_NOT_FOUND: Error = Error(25, "Transaction hash not found");

        pub struct Error(i64, &'static str);

        impl From<Error> for super::jsonrpc::Error {
            fn from(Error(code, message): Error) -> Self {
                Self {
                    code,
                    message: message.to_string(),
                }
            }
        }
    }
}
// ^^^ GENERATED CODE ABOVE ^^^

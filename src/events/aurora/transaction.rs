use aurora_engine_types::types::{Address, Wei};
use inindexer::near_indexer_primitives::types::BlockHeight;
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuroraTransactionEvent {
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,

    pub chain_id: Option<u64>,
    pub aurora_tx_hash: String,
    #[schemars(with = "String")]
    pub from: Address,
    #[schemars(with = "String")]
    pub to: Option<Address>,
    #[schemars(with = "String")]
    pub value: Wei,
    pub input: Vec<u8>,
    pub status: TransactionStatus,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum TransactionStatus {
    Succeed(Vec<u8>),
    Revert(Vec<u8>),
    OutOfGas,
    OutOfFund,
    OutOfOffset,
    CallTooDeep,
}

impl AuroraTransactionEvent {
    pub const ID: &'static str = "aurora_transaction";
}

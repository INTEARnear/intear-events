use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TxReceiptEvent {
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
    pub block_height: BlockHeight,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub predecessor_id: AccountId,
    #[schemars(with = "String")]
    pub executor_id: AccountId,
    pub success: Option<bool>,
}

impl TxReceiptEvent {
    pub const ID: &'static str = "tx_receipt";
}

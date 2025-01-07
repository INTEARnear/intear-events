use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TxTransactionEvent {
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
    pub block_height: BlockHeight,
    #[schemars(with = "String")]
    pub signer_id: AccountId,
    pub public_key: String,
    pub nonce: u64,
    #[schemars(with = "String")]
    pub receiver_id: AccountId,
    pub priority_fee: Option<u64>,
    pub signature: String,
    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
}

impl TxTransactionEvent {
    pub const ID: &'static str = "tx_transaction";
}

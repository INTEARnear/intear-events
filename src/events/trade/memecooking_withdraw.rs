use inindexer::near_indexer_primitives::types::{AccountId, Balance, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MemeCookingWithdrawEvent {
    pub meme_id: u64,
    #[schemars(with = "String")]
    pub trader: AccountId,
    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub amount: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub fee: Balance,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
}

impl MemeCookingWithdrawEvent {
    pub const ID: &'static str = "memecooking_withdraw";
}

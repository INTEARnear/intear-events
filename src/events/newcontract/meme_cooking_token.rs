use inindexer::near_indexer_primitives::types::{AccountId, Balance, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NewMemeCookingTokenEvent {
    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,

    pub meme_id: u64,
    #[schemars(with = "String")]
    pub token_id: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub total_supply: Balance,
    pub pool_id: u64,
}

impl NewMemeCookingTokenEvent {
    pub const ID: &'static str = "newtoken_memecooking_token";
}

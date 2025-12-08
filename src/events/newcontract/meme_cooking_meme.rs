use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::{dec_format, FtBalance};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NewMemeCookingMemeEvent {
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
    pub owner: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub end_timestamp_ms: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub total_supply: FtBalance,
    pub reference: String,
    pub reference_hash: String,
    #[schemars(with = "String")]
    pub deposit_token_id: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub soft_cap: FtBalance,
    #[serde(with = "dec_format")]
    #[schemars(with = "Option<String>")]
    pub hard_cap: Option<FtBalance>,
}

impl NewMemeCookingMemeEvent {
    pub const ID: &'static str = "newtoken_memecooking_meme";
}

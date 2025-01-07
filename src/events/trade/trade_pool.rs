use inindexer::near_indexer_primitives::types::{AccountId, Balance, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

type PoolId = String;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TradePoolEvent {
    pub pool: PoolId,
    #[schemars(with = "String")]
    pub token_in: AccountId,
    #[schemars(with = "String")]
    pub token_out: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub amount_in: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub amount_out: Balance,

    #[schemars(with = "String")]
    pub trader: AccountId,
    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
}

impl TradePoolEvent {
    pub const ID: &'static str = "trade_pool";
}

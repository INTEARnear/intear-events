use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BlockInfoEvent {
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
    #[schemars(with = "String")]
    pub block_hash: CryptoHash,
    #[schemars(with = "String")]
    pub block_producer: AccountId,
    pub transaction_count: u64,
    pub receipt_count: u64,
}

impl BlockInfoEvent {
    pub const ID: &'static str = "block_info";
}

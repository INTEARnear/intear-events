use inindexer::near_indexer_primitives::types::{AccountId, Balance, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::{dec_format, dec_format_vec};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NftTransferEvent {
    #[schemars(with = "String")]
    pub old_owner_id: AccountId,
    #[schemars(with = "String")]
    pub new_owner_id: AccountId,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,

    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<Option<String>>")]
    pub token_prices_near: Vec<Option<Balance>>,

    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
    #[schemars(with = "String")]
    pub contract_id: AccountId,
}

impl NftTransferEvent {
    pub const ID: &'static str = "nft_transfer";
}

use inindexer::{
    near_indexer_primitives::{
        types::{AccountId, BlockHeight},
        CryptoHash,
    },
    near_utils::{dec_format, FtBalance},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FtTransferEvent {
    #[schemars(with = "String")]
    pub old_owner_id: AccountId,
    #[schemars(with = "String")]
    pub new_owner_id: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub amount: FtBalance,
    pub memo: Option<String>,
    #[schemars(with = "String")]
    pub token_id: AccountId,

    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
}

impl FtTransferEvent {
    pub const ID: &'static str = "ft_transfer";
}

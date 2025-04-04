use chrono::{DateTime, Utc};
use inindexer::near_indexer_primitives::types::{AccountId, Balance, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{DonationId, ProjectId};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PotlockDonationEvent {
    pub donation_id: DonationId,
    #[schemars(with = "String")]
    pub donor_id: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub total_amount: Balance,
    #[schemars(with = "String")]
    pub ft_id: AccountId,
    pub message: Option<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[schemars(with = "u64")]
    pub donated_at: DateTime<Utc>,
    #[schemars(with = "String")]
    pub project_id: ProjectId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub protocol_fee: Balance,
    #[schemars(with = "Option<String>")]
    pub referrer_id: Option<AccountId>,
    #[serde(with = "dec_format")]
    #[schemars(with = "Option<String>")]
    pub referrer_fee: Option<Balance>,

    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
}

impl PotlockDonationEvent {
    pub const ID: &'static str = "potlock_donation";
}

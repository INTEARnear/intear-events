use chrono::{DateTime, Utc};
use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::{dec_format, FtBalance};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{DonationId, PotId};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PotlockPotDonationEvent {
    pub donation_id: DonationId,
    #[schemars(with = "String")]
    pub pot_id: PotId,
    #[schemars(with = "String")]
    pub donor_id: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub total_amount: FtBalance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub net_amount: FtBalance,
    pub message: Option<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[schemars(with = "u64")]
    pub donated_at: DateTime<Utc>,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub protocol_fee: FtBalance,
    #[schemars(with = "Option<String>")]
    pub referrer_id: Option<AccountId>,
    #[serde(with = "dec_format")]
    #[schemars(with = "Option<String>")]
    pub referrer_fee: Option<FtBalance>,
    #[schemars(with = "Option<String>")]
    pub chef_id: Option<AccountId>,
    #[serde(with = "dec_format")]
    #[schemars(with = "Option<String>")]
    pub chef_fee: Option<FtBalance>,

    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
}

impl PotlockPotDonationEvent {
    pub const ID: &'static str = "potlock_pot_donation";
}

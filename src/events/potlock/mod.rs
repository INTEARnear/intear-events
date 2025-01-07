use inindexer::near_indexer_primitives::types::AccountId;

pub mod potlock_donation;
pub mod potlock_pot_donation;
pub mod potlock_pot_project_donation;

type ProjectId = AccountId;
type PotId = AccountId;
type DonationId = u32;

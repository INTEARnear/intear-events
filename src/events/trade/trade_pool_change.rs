use inindexer::near_indexer_primitives::types::{AccountId, Balance, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::{dec_format, dec_format_vec};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

type PoolId = String;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TradePoolChangeEvent {
    pub pool_id: PoolId,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
    pub block_height: BlockHeight,
    pub pool: PoolType,
}

impl TradePoolChangeEvent {
    pub const ID: &'static str = "trade_pool_change";
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub enum PoolType {
    Ref(RefPool),
    Aidols(AidolsPool),
    GraFun(GraFunPool),
    Veax(VeaxPool),
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub enum RefPool {
    SimplePool(RefSimplePool),
    StableSwapPool(RefStableSwapPool),
    RatedSwapPool(RefRatedSwapPool),
    DegenSwapPool(RefDegenSwapPool),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct RefSimplePool {
    #[schemars(with = "Vec<String>")]
    pub token_account_ids: Vec<AccountId>,
    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<String>")]
    pub amounts: Vec<Balance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    pub exchange_fee: u32,
    pub referral_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: Balance,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct RefSwapVolume {
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub input: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub output: Balance,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct RefStableSwapPool {
    #[schemars(with = "Vec<String>")]
    pub token_account_ids: Vec<AccountId>,
    pub token_decimals: Vec<u8>,
    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<String>")]
    pub c_amounts: Vec<Balance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub init_amp_factor: u128,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub target_amp_factor: u128,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub init_amp_time: u64,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub stop_amp_time: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct RefRatedSwapPool {
    #[schemars(with = "Vec<String>")]
    pub token_account_ids: Vec<AccountId>,
    pub token_decimals: Vec<u8>,
    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<String>")]
    pub c_amounts: Vec<Balance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub init_amp_factor: u128,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub target_amp_factor: u128,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub init_amp_time: u64,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub stop_amp_time: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct RefDegenSwapPool {
    #[schemars(with = "Vec<String>")]
    pub token_account_ids: Vec<AccountId>,
    pub token_decimals: Vec<u8>,
    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<String>")]
    pub c_amounts: Vec<Balance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub init_amp_factor: u128,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub target_amp_factor: u128,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub init_amp_time: u64,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub stop_amp_time: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct AidolsPool {
    #[schemars(with = "String")]
    pub token_id: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub token_hold: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub wnear_hold: Balance,
    pub is_deployed: bool,
    pub is_tradable: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct GraFunPool {
    #[schemars(with = "String")]
    pub token_id: AccountId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub token_hold: Balance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub wnear_hold: Balance,
    pub is_deployed: bool,
    pub is_tradable: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[allow(dead_code)]
pub struct VeaxPool {
    #[schemars(with = "(String, String)")]
    pub pool: (AccountId, AccountId),
    pub r#type: String,
    /// 8 values, for each fee tier, lowest to highest
    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<String>")]
    pub amounts_a: Vec<Balance>,
    /// 8 values, for each fee tier, lowest to highest
    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<String>")]
    pub amounts_b: Vec<Balance>,
    pub sqrt_prices: Vec<f64>,
    pub liquidities: Vec<f64>,
}

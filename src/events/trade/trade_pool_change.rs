use std::fmt::{self, Display};
use std::str::FromStr;

use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::{dec_format, dec_format_vec, FtBalance};
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
    IntearPlach(IntearPlachPool),
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
    pub amounts: Vec<FtBalance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    pub exchange_fee: u32,
    pub referral_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: FtBalance,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct RefSwapVolume {
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub input: FtBalance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub output: FtBalance,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct RefStableSwapPool {
    #[schemars(with = "Vec<String>")]
    pub token_account_ids: Vec<AccountId>,
    pub token_decimals: Vec<u8>,
    #[serde(with = "dec_format_vec")]
    #[schemars(with = "Vec<String>")]
    pub c_amounts: Vec<FtBalance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: FtBalance,
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
    pub c_amounts: Vec<FtBalance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: FtBalance,
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
    pub c_amounts: Vec<FtBalance>,
    pub volumes: Vec<RefSwapVolume>,
    pub total_fee: u32,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub shares_total_supply: FtBalance,
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
    pub token_hold: FtBalance,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub wnear_hold: FtBalance,
    pub is_deployed: bool,
    pub is_tradable: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub enum IntearPlachPool {
    Private {
        assets: (IntearAssetWithBalance, IntearAssetWithBalance),
        fees: IntearPlachFeeConfiguration,
        #[schemars(with = "String")]
        owner_id: AccountId,
    },
    Public {
        assets: (IntearAssetWithBalance, IntearAssetWithBalance),
        fees: IntearPlachFeeConfiguration,
        #[serde(with = "dec_format")]
        #[schemars(with = "Option<String>")]
        total_shares: Option<FtBalance>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct IntearPlachFeeConfiguration {
    receivers: Vec<(IntearPlachFeeReceiver, IntearPlachFeeFraction)>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub enum IntearPlachFeeReceiver {
    Account(#[schemars(with = "String")] AccountId),
}

type IntearPlachFeeFraction = u32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct IntearAssetWithBalance {
    asset_id: IntearAssetId,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    balance: FtBalance,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IntearAssetId {
    Near,
    Nep141(AccountId),
    Nep245(AccountId, String),
    Nep171(AccountId, String),
}

impl Display for IntearAssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Near => write!(f, "near"),
            Self::Nep141(contract_id) => write!(f, "nep141:{contract_id}"),
            Self::Nep245(contract_id, token_id) => write!(f, "nep245:{contract_id}:{token_id}"),
            Self::Nep171(contract_id, token_id) => write!(f, "nep171:{contract_id}:{token_id}"),
        }
    }
}

impl FromStr for IntearAssetId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "near" => Ok(Self::Near),
            _ => match s.split_once(':') {
                Some(("nep141", contract_id)) => {
                    Ok(Self::Nep141(contract_id.parse().map_err(|e| {
                        format!("Invalid account id {contract_id}: {e}")
                    })?))
                }
                Some(("nep245", rest)) => {
                    if let Some((contract_id, token_id)) = rest.split_once(':') {
                        Ok(Self::Nep245(
                            contract_id
                                .parse()
                                .map_err(|e| format!("Invalid account id {contract_id}: {e}"))?,
                            token_id.to_string(),
                        ))
                    } else {
                        Err(format!("Invalid asset id: {s}"))
                    }
                }
                Some(("nep171", rest)) => {
                    if let Some((contract_id, token_id)) = rest.split_once(':') {
                        Ok(Self::Nep171(
                            contract_id
                                .parse()
                                .map_err(|e| format!("Invalid account id {contract_id}: {e}"))?,
                            token_id.to_string(),
                        ))
                    } else {
                        Err(format!("Invalid asset id: {s}"))
                    }
                }
                _ => Err(format!("Invalid asset id: {s}")),
            },
        }
    }
}

impl Serialize for IntearAssetId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IntearAssetId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl schemars::JsonSchema for IntearAssetId {
    fn schema_name() -> String {
        "AssetId".to_string()
    }
    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as schemars::JsonSchema>::json_schema(generator)
    }
}

use inindexer::near_indexer_primitives::types::AccountId;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PricePoolEvent {
    pub pool_id: String,
    #[schemars(with = "String")]
    pub token0: AccountId,
    #[schemars(with = "String")]
    pub token1: AccountId,
    #[serde(with = "stringified")]
    #[schemars(with = "String")]
    pub token0_in_1_token1: BigDecimal,
    #[serde(with = "stringified")]
    #[schemars(with = "String")]
    pub token1_in_1_token0: BigDecimal,

    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub timestamp_nanosec: u128,
}

impl PricePoolEvent {
    pub const ID: &'static str = "price_pool";
}

mod stringified {
    use serde::Deserialize;

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: ToString,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(serde::de::Error::custom)
    }
}

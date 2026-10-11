use bigdecimal::BigDecimal;
use inindexer::near_indexer_primitives::types::AccountId;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PriceTokenEvent {
    #[schemars(with = "String")]
    pub token: AccountId,
    #[serde(with = "stringified")]
    #[schemars(with = "String")]
    pub price_usd: BigDecimal,

    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub timestamp_nanosec: u128,
}

impl PriceTokenEvent {
    pub const ID: &'static str = "price_token";
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

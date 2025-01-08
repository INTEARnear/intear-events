use std::{collections::HashMap, str::FromStr};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use inevents::{actix_web::http::StatusCode, events::event::HttpEndpoint};
use inindexer::near_indexer_primitives::types::AccountId;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::BigDecimal, PgPool};

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

pub struct OhlcEndpoint;

#[async_trait]
impl HttpEndpoint for OhlcEndpoint {
    async fn handle(
        &self,
        pool: PgPool,
        query: HashMap<String, String>,
    ) -> (StatusCode, serde_json::Value) {
        let Some(token) = query.get("token") else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({"error": "token is required"}),
            );
        };
        let Ok(token) = AccountId::from_str(token) else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "invalid token"
                }),
            );
        };
        let Some(resolution) = query.get("resolution") else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "resolution is required"
                }),
            );
        };
        let Ok(resolution) = OhlcResolution::from_str(resolution) else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "invalid resolution"
                }),
            );
        };
        let Some(count_back) = query.get("count_back") else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "count_back is required"
                }),
            );
        };
        let Ok(count_back) = count_back.parse::<usize>() else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "invalid count_back"
                }),
            );
        };
        const MAX_COUNT_BACK: usize = 15000;
        if count_back > MAX_COUNT_BACK {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": format!("count_back must be less than or equal to {MAX_COUNT_BACK}")
                }),
            );
        }
        let Some(to) = query.get("to") else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "to is required"
                }),
            );
        };
        let Ok(to) = to.parse::<i64>() else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "invalid to"
                }),
            );
        };
        let Some(to) = chrono::DateTime::from_timestamp_millis(to) else {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "error": "invalid to"
                }),
            );
        };

        #[derive(Debug, FromRow)]
        struct OhlcBar {
            bucket: DateTime<Utc>,
            #[allow(dead_code)] // replaced with previous bucket's close
            open: BigDecimal,
            high: BigDecimal,
            low: BigDecimal,
            close: BigDecimal,
        }

        macro_rules! query_materialized_view {
            ($q: literal) => {
                sqlx::query_as::<_, OhlcBar>($q)
                .bind(token.to_string())
                .bind(count_back as i64 + 1)
                .bind(to)
                .fetch_all(&pool)
                .await
                .map(|records| {
                    let mut bars = Vec::new();
                    let mut records = records.into_iter().rev();
                    let Some(first_bar) = records.next() else {
                        return bars;
                    };
                    let mut prev_close = first_bar.close;
                    for record in records {
                        let high = record.high.max(prev_close.clone());
                        let low = record.low.min(prev_close.clone());
                        bars.push(serde_json::json!({
                            "timestamp_millis": record.bucket.timestamp_millis(),
                            "open": prev_close.with_prec(42).to_string(),
                            "high": high.with_prec(42).to_string(),
                            "low": low.with_prec(42).to_string(),
                            "close": record.close.clone().with_prec(42).to_string(),
                        }));
                        prev_close = record.close;
                    }
                    bars
                })
                .map_err(|err| {
                    log::error!("Error querying OHLC data: {:?}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        serde_json::json!({"error": "internal server error"}),
                    )
                })
            };
        }

        let results = match resolution {
            OhlcResolution::OneMinute => query_materialized_view!(
                r#"
                    SELECT bucket, open, high, low, close
                    FROM price_token_1min_ohlc
                    WHERE token = $1
                    AND bucket < $3
                    ORDER BY bucket DESC
                    LIMIT $2;
                    "#
            ),
            OhlcResolution::OneHour => query_materialized_view!(
                r#"
                    SELECT bucket, open, high, low, close
                    FROM price_token_1hour_ohlc
                    WHERE token = $1
                    AND bucket < $3
                    ORDER BY bucket DESC
                    LIMIT $2;
                    "#
            ),
            OhlcResolution::OneDay => query_materialized_view!(
                r#"
                    SELECT bucket, open, high, low, close
                    FROM price_token_1day_ohlc
                    WHERE token = $1
                    AND bucket < $3
                    ORDER BY bucket DESC
                    LIMIT $2;
                    "#
            ),
        };
        let results = match results {
            Ok(results) => results,
            Err((status, error)) => return (status, error),
        };

        (
            StatusCode::OK,
            serde_json::to_value(&results).expect("Error serializing OHLC response"),
        )
    }
}

#[allow(clippy::enum_variant_names)]
enum OhlcResolution {
    OneMinute,
    OneHour,
    OneDay,
}

impl FromStr for OhlcResolution {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::OneMinute),
            "60" => Ok(Self::OneHour),
            "1D" => Ok(Self::OneDay),
            _ => Err(()),
        }
    }
}

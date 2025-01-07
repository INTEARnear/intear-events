use inindexer::near_indexer_primitives::types::{AccountId, BlockHeight};
use inindexer::near_indexer_primitives::CryptoHash;
use inindexer::near_utils::dec_format;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TradeSwapEvent {
    #[serde(with = "stringified_map")]
    #[schemars(with = "HashMap<String, String>")]
    pub balance_changes: HashMap<AccountId, i128>,

    #[schemars(with = "String")]
    pub trader: AccountId,
    #[schemars(with = "String")]
    pub transaction_id: CryptoHash,
    #[schemars(with = "String")]
    pub receipt_id: CryptoHash,
    pub block_height: BlockHeight,
    #[serde(with = "dec_format")]
    #[schemars(with = "String")]
    pub block_timestamp_nanosec: u128,
}

impl TradeSwapEvent {
    pub const ID: &'static str = "trade_swap";
}

mod stringified_map {
    use std::collections::HashMap;
    use std::str::FromStr;

    use serde::de::MapAccess;
    use serde::ser::SerializeMap;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, K, V>(value: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        K: Serialize,
        V: ToString,
    {
        let mut map = serializer.serialize_map(Some(value.len()))?;
        for (k, v) in value {
            map.serialize_entry(k, &v.to_string())?;
        }
        map.end()
    }

    struct Visitor<T, K, V>(
        std::marker::PhantomData<T>,
        std::marker::PhantomData<K>,
        std::marker::PhantomData<V>,
    );

    impl<'de, T, K, V> de::Visitor<'de> for Visitor<T, K, V>
    where
        K: Deserialize<'de>,
        V: FromStr,
        V::Err: std::fmt::Display,
        T: FromIterator<(K, V)>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut items = Vec::with_capacity(map.size_hint().unwrap_or(0));
            while let Some((key, value)) = map.next_entry::<K, String>()? {
                items.push((key, V::from_str(&value).map_err(serde::de::Error::custom)?));
            }
            Ok(items.into_iter().collect())
        }
    }

    pub fn deserialize<'de, T, K, V, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromIterator<(K, V)>,
        K: Deserialize<'de>,
        V: FromStr,
        V::Err: std::fmt::Display,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Visitor(
            std::marker::PhantomData,
            std::marker::PhantomData,
            std::marker::PhantomData,
        ))
    }
}

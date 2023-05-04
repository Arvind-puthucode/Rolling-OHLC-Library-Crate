use anyhow::Result;
use serde::{self, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Ohlc {
    pub symbol: String,
    pub timestamp: i64,
    #[serde(serialize_with = "back_to_json")]
    pub open: f64,
    #[serde(serialize_with = "back_to_json")]
    pub high: f64,
    #[serde(serialize_with = "back_to_json")]
    pub low: f64,
    #[serde(serialize_with = "back_to_json")]
    pub close: f64,
}

fn back_to_json<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
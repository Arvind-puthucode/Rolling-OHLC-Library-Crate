use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct InputStruct {
    e: String,
    u: i64,
    pub(crate) s: String,
    #[serde(deserialize_with = "json_to_val")]
    pub(crate) b: f64,
    #[serde(deserialize_with = "json_to_val")]
    B: f64,
    #[serde(deserialize_with = "json_to_val")]
    pub(crate) a: f64,
    #[serde(deserialize_with = "json_to_val")]
    A: f64,
    pub(crate) T: i64,
    E: i64,
}

impl FromStr for InputStruct {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


fn json_to_val<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}


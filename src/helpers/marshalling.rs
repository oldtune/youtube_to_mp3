use serde::{Deserialize, Serialize};

use crate::err::Result;

pub fn deserialize<'a, T>(s: &'a str) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    let result: T = serde_json::from_str(s)?;
    Ok(result)
}

pub fn serialize<T>(something: T) -> Result<String>
where
    T: serde::ser::Serialize,
{
    let result = serde_json::to_string(&something)?;
    Ok(result)
}

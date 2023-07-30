use std::fmt::Debug;

use serde::{Deserialize, Serialize, de::Error};

#[derive(PartialEq, Eq)]
pub enum AttemptResult {
    Skipped,
    DNF,
    DNS,
    Ok(u32),
}

impl Serialize for AttemptResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_i32(match self {
            AttemptResult::Skipped => 0,
            AttemptResult::DNF => -1,
            AttemptResult::DNS => -2,
            AttemptResult::Ok(v) => *v as i32,
        })
    }
}

impl<'de> Deserialize<'de> for AttemptResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        Ok(match i32::deserialize(deserializer)? {
            -2 => AttemptResult::DNS,
            -1 => AttemptResult::DNF,
            0 => AttemptResult::Skipped,
            v if v > 0 => AttemptResult::Ok(v as u32),
            v => Err(D::Error::custom(format!("{v} is not a valid attempt result")))?,
        })
    }
}

impl Debug for AttemptResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttemptResult::Skipped => f.write_str("Skipped"),
            AttemptResult::DNF => f.write_str("DNF"),
            AttemptResult::DNS => f.write_str("DNS"),
            AttemptResult::Ok(v) => f.write_str(&v.to_string()),
        }
    }
}

use serde::{Serialize, Deserialize};

pub enum AttemptResult {
    Skipped,
    DNF,
    DNS,
    Ok(u64),
}

impl Serialize for AttemptResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_i64(match self {
            AttemptResult::Skipped => 0,
            AttemptResult::DNF => -1,
            AttemptResult::DNS => -2,
            AttemptResult::Ok(v) => v as i64,
        })
    }
}

impl<'de> Deserialize<'de> for AttemptResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(match i64::deserialize(deserializer)? {
            -2 => AttemptResult::DNS,
            -1 => AttemptResult::DNF,
            0 => AttemptResult::Skipped,
            v if v > 0 => AttemptResult::Ok(v),
            v => Err(D::Error::custom(format!("{v} is not a valid attempt result"))),
        })
    }
}

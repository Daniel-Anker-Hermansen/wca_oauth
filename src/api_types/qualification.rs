use serde::{Deserialize, Serialize};

use super::{Date, AttemptResult};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Qualification {
    #[serde(rename_all = "camelCase")]
    AttemptResult {
        when_date: Date,
        result_type: String,
        level: AttemptResult,
    },
    #[serde(rename_all = "camelCase")]
    Ranking {
        when_date: Date,
        result_type: String,
        level: u32,
    },
    #[serde(rename_all = "camelCase")]
    AnyResult {
        when_date: Date,
        result_type: String,
    }
}

// WIP to replace on top ... TODO
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResultType {
    Single,
    Average,
}

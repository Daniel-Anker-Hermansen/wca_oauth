use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimeLimit {
    pub centiseconds: u32,
    pub cumulative_round_ids: Vec<String>,
}

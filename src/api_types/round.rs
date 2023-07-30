use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{TimeLimit, Cutoff, AdvancementCondition, Result, ScrambleSet};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub id: String,
    pub format: String,
    pub time_limit: Option<TimeLimit>,
    pub cutoff: Option<Cutoff>,
    pub advancement_condition: Option<AdvancementCondition>,
    pub results: Vec<Result>,
    pub scramble_set_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scramble_sets: Option<Vec<ScrambleSet>>,
    pub extensions: Vec<Value>,
}

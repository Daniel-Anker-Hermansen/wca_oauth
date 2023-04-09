use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub id: String,
    pub format: String,
    pub time_limit: Option<TimeLimit>,
    pub cutoff: Option<Cutoff>,
    pub advancement_condition: Option<AdvancementCondition>,
    pub result: Vec<Result>,
    pub scramble_set_count: u64,
    pub scramble_sets: Vec<ScrambleSet>,
    pub extentions: Vec<Value>,
}

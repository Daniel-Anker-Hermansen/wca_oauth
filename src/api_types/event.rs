use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub rounds: Vec<Round>,
    pub competitor_limit: Option<u64>,
    pub qualification: Option<Qualification>,
    pub extensions: Vec<Value>,
}

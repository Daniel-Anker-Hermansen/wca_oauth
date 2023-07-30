use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Round, Qualification};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub rounds: Vec<Round>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitor_limit: Option<u32>,
    pub qualification: Option<Qualification>,
    pub extensions: Vec<Value>,
}

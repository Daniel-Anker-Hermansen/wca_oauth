use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Activity;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub activities: Vec<Activity>,
    pub extensions: Vec<Value>,
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::DateTime;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: u32,
    pub name: String,
    // TODO better activity code.
    pub activity_code: String,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub child_activities: Vec<Activity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scramble_set_id: Option<u32>,
    pub extensions: Value,
}

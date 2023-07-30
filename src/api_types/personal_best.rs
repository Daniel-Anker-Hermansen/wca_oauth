use serde::{Deserialize, Serialize};

use super::AttemptResult;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonalBest {
    pub event_id: String,
    pub best: AttemptResult,
    pub r#type: String,
    pub world_ranking: u32,
    pub continental_ranking: u32,
    pub national_ranking: u32,
}

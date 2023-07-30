use serde::{Deserialize, Serialize};

use super::AttemptResult;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum AdvancementCondition {
    #[serde(rename_all = "camelCase")]
    Ranking { level: u32 },
    #[serde(rename_all = "camelCase")]
    Percent { level: u32 },
    #[serde(rename_all = "camelCase")]
    AttemptResult { level: AttemptResult },
}

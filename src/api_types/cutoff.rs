use serde::{Serialize, Deserialize};

use super::AttemptResult;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Cutoff {
    pub number_of_attempts: u32,
    pub attempt_result: AttemptResult,
}

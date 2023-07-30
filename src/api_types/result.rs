use serde::{Deserialize, Serialize};

use super::{AttemptResult, Attempt};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub person_id: u32,
    pub ranking: Option<u32>,
    pub attempts: Vec<Attempt>,
    pub best: AttemptResult,
    pub average: AttemptResult,
}

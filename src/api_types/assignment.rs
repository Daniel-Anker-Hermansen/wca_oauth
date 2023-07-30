use super::AssignmentCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub activity_id: u32,
    pub assignment_code: AssignmentCode,
    pub station_number: Option<u32>,
}

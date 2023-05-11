use serde::{Serialize, Deserialize};
use super::AssignmentCode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub activity_id: i64,
    pub assignment_code: AssignmentCode,
    pub station_number: Option<i64>,
}

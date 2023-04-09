use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub activity_id: i64,
    pub assignment_code: AssignmentCode,
    pub station_number: Option<i64>,
}

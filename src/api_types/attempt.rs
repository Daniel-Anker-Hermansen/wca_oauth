use serde::{Deserialize, Serialize};

use super::AttemptResult;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Attempt {
    pub result: AttemptResult,
    // string is wrapped in a box as this field is never used. This saves 16 bytes per attempt on
    // 64-bit systems.
    pub reconstruction: Option<Box<String>>,
}

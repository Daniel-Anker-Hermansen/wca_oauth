use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScrambleSet {
    pub id: u32,
    pub scrambles: Vec<String>,
    pub extra_scrambles: Vec<String>,
}

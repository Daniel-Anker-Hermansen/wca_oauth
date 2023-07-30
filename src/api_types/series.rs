use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub competition_ids: Vec<String>,
}

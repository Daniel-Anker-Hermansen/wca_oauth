use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub url: String,
    pub thumb_url: String,
}

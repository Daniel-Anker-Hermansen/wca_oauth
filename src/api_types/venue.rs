use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Room;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: u32,
    pub name: String,
    pub latitude_microdegrees: i32,
    pub longitude_microdegrees: i32,
    pub country_iso2: String,
    pub timezone: String,
    pub rooms: Vec<Room>,
    pub extensions: Vec<Value>,
}

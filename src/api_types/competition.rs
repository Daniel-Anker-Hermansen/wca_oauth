use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Competition<T> {
    pub format_version: String,
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub series: Option<Series>,
    pub persons: Vec<Person<T>>,
    pub events: Vec<Event>,
    pub schedule: Schedule,
    pub competitor_limit: Option<u64>,
    pub extensions: Vec<Value>,
}

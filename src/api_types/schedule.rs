use serde::{Deserialize, Serialize};

use super::{Date, Venue};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub start_date: Date,
    pub number_of_days: u32,
    pub venues: Vec<Venue>,
}

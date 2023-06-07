use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::Enabled;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Registration<Vis> {
        pub wca_registrant_id: u64,
        pub event_ids: Vec<String>,
        pub status: String,
        guests: Option<u64>,
        comments: Option<String>,
        _phantom: PhantomData<Vis>,
}

impl Registration<Enabled> {
        pub fn guests(&self) -> u64 {
                self.guests.unwrap()
        }

        pub fn comments(&self) -> &str {
                &self.comments.unwrap()
        }
}

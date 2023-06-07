use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonalBest {
        pub event_id: String,
        pub best: AttemptResult,
        pub r#type: String,
        pub world_ranking: u64,
        pub continental_ranking: u64,
        pub national_ranking: u64,
}

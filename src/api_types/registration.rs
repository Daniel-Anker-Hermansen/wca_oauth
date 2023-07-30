use serde::{Deserialize, Serialize};

use crate::ScopeTypes;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Registration<Vis: ScopeTypes> {
    pub wca_registration_id: u32,
    pub event_ids: Vec<String>,
    pub status: String,
    pub guests: Vis::Guests,
    pub comments: Vis::Comments,
    pub administrative_notes: Vis::Comments,
    pub is_competing: bool,
}

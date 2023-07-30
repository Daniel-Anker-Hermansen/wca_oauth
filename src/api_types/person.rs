use serde::{Deserialize, Serialize};

use crate::ScopeTypes;

use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Person<Vis: ScopeTypes> {
    pub registrant_id: Option<u32>,
    pub name: String,
    pub wca_user_id: u32,
    pub wca_id: Option<String>,
    pub country_iso_2: String,
    pub gender: char,
    pub birthdate: Vis::DateOfBirth,
    pub email: Vis::Email,
    pub avatar: Option<Avatar>,
    pub roles: Vec<Role>,
    pub registration: Option<Registration<Vis>>,
    pub assignments: Vec<Assignment>,
    pub personal_bests: Vec<PersonalBest>,
    pub extensions: Vec<Value>,
}

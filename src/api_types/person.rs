use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use crate::Enabled;

use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person<Vis> {
    pub registrant_id: Option<u64>,
    pub name: String,
    pub wca_user_id: u64,
    pub wca_id: Option<String>,
    pub country_iso_2: String,
    pub gender: char,
    birthdate: Option<Date>,
    email: Option<String>,
    pub avatar: Option<Avatar>,
    pub roles: Vec<Role>,
    pub registration: Option<Registration<Vis>>,
    pub assignments: Vec<Assignment>,
    pub personal_bests: Vec<PersonalBest>,
    pub extensions: Vec<Value>,
    _phantom: PhantomData<Vis>,
}

impl Person<Enabled> {
    pub fn birthdate(&self) -> &Date {
        &self.birthdate.unwrap()
    }

    pub fn email(&self) -> &str {
        &self.email.unwrap()
    }
}

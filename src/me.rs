use serde::{Deserialize, Serialize};

use crate::ScopeTypes;

#[derive(Deserialize, Serialize, Debug)]
pub struct Me<Email: ScopeTypes, DateOfBirth: ScopeTypes> {
        email: Email::Email,
        dob: DateOfBirth::DateOfBirth,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct MeWrapper<Email, DateOfBirth> {
    pub me: Me<Email, DateOfBirth>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Me<Email, DateOfBirth> {
    email: Email,
    dob: DateOfBirth,
}

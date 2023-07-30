use std::ops::{Deref, DerefMut};

use reqwest::Method;
use serde::{Serialize, Deserialize};

use super::Competition;

use crate::{Disabled, Enabled, Client, Error};

pub struct PublicWcif {
    pub(crate) inner: Competition<Disabled>,
}

impl Deref for PublicWcif {
    type Target = Competition<Disabled>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PublicWcif {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct PrivateWcif {
    pub(crate) inner: Competition<Enabled>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchStatus {
    pub status: String,
}

impl PrivateWcif {
    pub async fn patch<T>(&self, client: &T) -> Result<PatchStatus, Error> where T: Client<ManageCompetitions = Enabled> + Send + Sync {
        let result = client.request_builder(&format!("competitions/{}/wcif", self.id), Method::PATCH)
            .body(serde_json::to_string(&self.inner).unwrap())
            .header("Content-Type", "application/json")
            .send()
            .await?
            .text()
            .await?;
        crate::oauth::parse_json(&result)
    }
}

impl Deref for PrivateWcif {
    type Target = Competition<Enabled>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PrivateWcif {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

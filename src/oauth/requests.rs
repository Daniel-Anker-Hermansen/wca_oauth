use std::{collections::HashMap, num::NonZeroUsize};

use reqwest::Method;

use crate::{
    me::{Me, MeWrapper},
    Competition, api_types::{PublicWcif, PrivateWcif},
};

use super::*;

pub struct CompetitionsEndpoint<'a, T>
where T: ?Sized {
    query: HashMap<&'static str, String>,
    inner: &'a T,
}

impl<T> CompetitionsEndpoint<'_, T>
where T: Client + ?Sized + Send + Sync
{
    pub(super) fn new(inner: &T) -> CompetitionsEndpoint<'_, T> {
        CompetitionsEndpoint {
            query: HashMap::new(),
            inner,
        }
    }

    pub async fn send(&self) -> Result<Vec<Competition>, Error> {
        let url = format!(
            "competitions?{}",
            self.query
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>() // replace with intersperce when stable
                .join("&")
        );
        let json = self.inner.custom_route(&url, Method::GET).await?;
        parse_json(&json)
    }

    /// Sets the managed_by_me flag to true, and only shows competitions at which you have
    /// administrator privileges. Requires manage_competitions scope.
    pub fn managed_by_me(&mut self) -> &mut Self
    where T: Client<ManageCompetitions = Enabled> {
        self.query
            .insert("managed_by_me", "true".to_owned());
        self
    }

    /// Chooses which page to request. Default is 1. Each page has at most 25 entries.
    /// Note: Pages are 1-indexed.
    pub fn page(&mut self, index: NonZeroUsize) -> &mut Self {
        self.query
            .insert("page", index.to_string());
        self
    }
}

pub struct MeEndpoint<'a, T>
where T: ?Sized {
    inner: &'a T,
}

impl<T> MeEndpoint<'_, T>
where T: Client + ?Sized + Send + Sync
{
    pub(super) fn new(inner: &T) -> MeEndpoint<'_, T> {
        MeEndpoint {
            inner,
        }
    }

    pub async fn send(
        &self,
    ) -> Result<Me<<T::Email as ScopeTypes>::Email, <T::DateOfBirth as ScopeTypes>::DateOfBirth>, Error> {
        let json = self.inner.custom_route("me", Method::GET).await?;
        parse_json::<MeWrapper<_, _>>(&json).map(|m| m.me)
    }
}

/// Parses the json to the type T if possible, otherwise it parses to ApiError. If that fails it
/// returns an other varaint eroor with the src json.
pub(crate) fn parse_json<'de, T>(json: &'de str) -> Result<T, Error>
where T: Deserialize<'de> {
    serde_json::from_str(&json).map_err(|_| {
        serde_json::from_str::<ApiError>(&json)
            .map(|api_error| api_error.into())
            .unwrap_or_else(|e| Error::Serde(e))
    })
}

pub struct PublicWcifEndpoint<'a, T>
where T: ?Sized {
    inner: &'a T,
    competition_name: &'a str,
}

impl<T> PublicWcifEndpoint<'_, T>
where T: Client + ?Sized + Send + Sync
{
    pub(super) fn new<'a>(inner: &'a T, competition_name: &'a str) -> PublicWcifEndpoint<'a, T> {
        PublicWcifEndpoint { inner, competition_name }
    }

    pub async fn send(&self) -> Result<PublicWcif, Error> {
        let json = self.inner.custom_route(&format!("competitions/{}/wcif/public", self.competition_name), Method::GET).await?;
        parse_json::<crate::api_types::Competition<Disabled>>(&json).map(|wcif| PublicWcif { inner: wcif })
    }
}

pub struct PrivateWcifEndpoint<'a, T>
where T: ?Sized {
    inner: &'a T,
    competition_name: &'a str,
}

impl<T> PrivateWcifEndpoint<'_, T>
where T: Client + ?Sized + Send + Sync
{
    pub(super) fn new<'a>(inner: &'a T, competition_name: &'a str) -> PrivateWcifEndpoint<'a, T> {
        PrivateWcifEndpoint { inner, competition_name }
    }

    pub async fn send(&self) -> Result<PrivateWcif, Error> {
        let json = self.inner.custom_route(&format!("competitions/{}/wcif", self.competition_name), Method::GET).await?;
        parse_json::<crate::api_types::Competition<Enabled>>(&json).map(|wcif| PrivateWcif { inner: wcif })
    }
}

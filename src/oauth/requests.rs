use std::{collections::HashMap, num::NonZeroUsize};

use crate::{me::Me, Competition};

use super::*;

pub struct CompetitionsEndpoint<'a, T>
where T: ?Sized {
        query: HashMap<&'static str, String>,
        inner: &'a T,
}

impl<T> CompetitionsEndpoint<'_, T>
where T: Client + ?Sized
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
                let json = self.inner.custom_route(&url).await?;
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
where T: Client + ?Sized
{
        pub(super) fn new(inner: &T) -> MeEndpoint<'_, T> {
                MeEndpoint {
                        inner,
                }
        }

        pub async fn send(&self) -> Result<Me<T::Email, T::DateOfBirth>, Error> {
                let json = self.inner.custom_route("me").await?;
                parse_json(&json)
        }
}

/// Parses the json to the type T if possible, otherwise it parses to ApiError. If that fails it
/// returns an other varaint eroor with the src json.
pub(super) fn parse_json<'de, T>(json: &'de str) -> Result<T, Error>
where T: Deserialize<'de> {
        serde_json::from_str(&json).map_err(|_| {
                serde_json::from_str::<ApiError>(&json)
                        .map(|api_error| api_error.into())
                        .unwrap_or_else(|_| Error::Other(json.to_owned()))
        })
}

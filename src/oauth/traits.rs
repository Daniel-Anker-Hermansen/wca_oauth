use super::*;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
#[allow(unused_imports)]
use serde::{de::Error as _, Serialize};

/// The special WCA loopback uri, useful in early development
pub const LOOPBACK_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

pub trait ScopeTypes {
        type Email: DeserializeOwned + Serialize;

        type DateOfBirth: DeserializeOwned + Serialize;
}

#[derive(Debug)]
pub struct Enabled;

#[derive(Debug)]
pub struct Disabled;

impl ScopeTypes for Enabled {
        type Email = String;

        type DateOfBirth = String;
}

/// A type which can never exist because ! is unstable.
#[derive(Debug)]
pub struct Never;

impl<'de> Deserialize<'de> for Never {
        fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
                Ok(Never)
        }
}

impl Serialize for Never {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
                serializer.serialize_none()
        }
}

impl ScopeTypes for Disabled {
        type Email = Never;

        type DateOfBirth = Never;
}

/// Main trait for an oauth instance
/// An implementation of this can be obtained through an OAuthBuilder
#[async_trait]
pub trait Client {
        type Email: ScopeTypes;

        type ManageCompetitions: ScopeTypes;

        type DateOfBirth: ScopeTypes;

        type Public: ScopeTypes;

        fn prefix(&self) -> &str;

        fn client(&self) -> &reqwest::Client;

        async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error>;

        fn competitions(&self) -> CompetitionsEndpoint<'_, Self> {
                CompetitionsEndpoint::new(self)
        }

        fn me(&self) -> MeEndpoint<'_, Self>
        where Self: Client<Public = Enabled> {
                MeEndpoint::new(self)
        }
}

/// Builder trait for building an oauth instance
pub trait ClientBuilder: Sized + Clone {
        type ImplicitClient<'a>: Client + Sync + Send;

        fn with_secret(self, client_id: String, secret: String, redirect_uri: String) -> WithSecret<Self> {
                WithSecret {
                        client_id,
                        secret,
                        redirect_uri,
                        inner: self,
                        url: "https://staging.worldcubeassociation.org/oauth/token".to_owned(),
                }
        }

        fn with_manage_competition(self) -> ScopedClientBuilder<ManageCompetititons, Self> {
                ScopedClientBuilder::new(self)
        }

        fn with_email(self) -> ScopedClientBuilder<Email, Self> {
                ScopedClientBuilder::new(self)
        }

        fn with_dob(self) -> ScopedClientBuilder<DateOfBirth, Self> {
                ScopedClientBuilder::new(self)
        }

        fn with_public(self) -> ScopedClientBuilder<Public, Self> {
                ScopedClientBuilder::new(self)
        }

        fn scopes(&self) -> Vec<&str>;

        fn authenticate_implicit(self, access_token: String) -> Self::ImplicitClient<'static> {
                self.authenticate_implicit_with_client(access_token, &CLIENT)
        }

        fn authenticate_implicit_with_client<'a>(
                self, access_token: String, client: &'a reqwest::Client,
        ) -> Self::ImplicitClient<'a>;
}

pub(super) trait SetAccessToken {
        fn set_access_token(&mut self, access_token: String);
}

#[async_trait]
pub trait RefreshableClient {
        fn scopes(&self) -> Vec<&str>;

        async fn refresh(&mut self) -> Result<(), Error>;
}

#[async_trait]
pub trait ClientWithSecretBuilder: Sized + ClientBuilder + Clone {
        type ExplicitClient<'a>: Client + RefreshableClient + Sync + Send;

        async fn authenticate_explicit(self, access_code: String) -> Result<Self::ExplicitClient<'static>, Error> {
                self.authenticate_explicit_with_client(access_code, &CLIENT)
                        .await
        }

        async fn authenticate_explicit_with_client<'a>(
                self, access_code: String, client: &'a reqwest::Client,
        ) -> Result<Self::ExplicitClient<'a>, Error>;
}

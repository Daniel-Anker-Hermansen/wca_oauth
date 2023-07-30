use crate::api_types::Date;

use super::*;
use async_trait::async_trait;
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
#[allow(unused_imports)]
use serde::{de::Error as _, Serialize};
use std::fmt::Debug;

/// The special WCA loopback uri, useful in early development
pub const LOOPBACK_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

pub trait ScopeTypes {
    type Email: DeserializeOwned + Serialize + Debug + Eq + PartialEq;

    type DateOfBirth: DeserializeOwned + Serialize + Debug + Eq + PartialEq;

    type Guests: DeserializeOwned + Serialize + Debug + Eq + PartialEq;
    
    type Comments: DeserializeOwned + Serialize + Debug + Eq + PartialEq;
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Enabled;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Disabled;

impl ScopeTypes for Enabled {
    type Email = String;

    type DateOfBirth = Date;

    type Guests = u32;

    type Comments = String;
}

impl ScopeTypes for Disabled {
    type Email = Unavailable;

    type DateOfBirth = Unavailable;

    type Guests = Unavailable;

    type Comments = Unavailable;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Unavailable;

impl<'de> Deserialize<'de> for Unavailable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        // Serialize whatever value might be there and throw it away.
        Option::<serde_json::Value>::deserialize(deserializer).map(|_| Unavailable)
    }
}

impl Serialize for Unavailable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        // Serializing none in json leads to the field being removed which is what we
        // want.
        serializer.serialize_none()
    }
}

/// Main trait for an oauth instance
/// An implementation of this can be obtained through an OAuthBuilder
#[async_trait]
pub trait Client: Send + Sync {
    type Email: ScopeTypes;

    type ManageCompetitions: ScopeTypes;

    type DateOfBirth: ScopeTypes;

    type Public: ScopeTypes;

    fn prefix(&self) -> &str;

    fn client(&self) -> &reqwest::Client;

    fn request_builder(&self, suffix: &str, method: Method) -> RequestBuilder;

    async fn custom_route(&self, suffix: &str, method: Method) -> Result<String, reqwest::Error> {
        self.request_builder(suffix, method)
            .send()
            .await?
            .text()
            .await
    }

    fn competitions(&self) -> CompetitionsEndpoint<'_, Self> {
        CompetitionsEndpoint::new(self)
    }

    fn me(&self) -> MeEndpoint<'_, Self>
    where Self: Client<Public = Enabled> {
        MeEndpoint::new(self)
    }

    fn public_wcif<'a>(&'a self, competition_name: &'a str) -> PublicWcifEndpoint<'a, Self> {
        PublicWcifEndpoint::new(self, competition_name)
    }

    fn private_wcif<'a>(&'a self, competition_name: &'a str) -> PrivateWcifEndpoint<'a, Self> 
    where Self: Client<ManageCompetitions = Enabled> {
        PrivateWcifEndpoint::new(self, competition_name)
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

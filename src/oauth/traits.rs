use std::collections::HashMap;

use async_trait::async_trait;
use super::*;

/// The special WCA loopback uri, useful in early development
pub const LOOPBACK_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

/// Main trait for an oauth instance
/// An implementation of this can be obtained through an OAuthBuilder
#[async_trait]
pub trait OAuth {
    type Email: Scope;

    type ManageCompetitions: Scope;
    
    type DateOfBirth: Scope;

    fn competitions(&self) -> CompetitionsEndpoint<'_, Self> {
        CompetitionsEndpoint { query: HashMap::new(), inner: self }
    }

    fn prefix(&self) -> &str;
    
    fn client(&self) -> &Client;

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error>;
}

pub(super) trait SetAccessToken {
    fn set_access_token(&mut self, access_token: String);
}

pub trait LoggedIn: OAuth { 
    fn me(&self) -> MeEndpoint<'_, Self> where Self: LoggedIn {
        MeEndpoint { inner: self }
    }    
}

#[async_trait]
pub trait Refreshable { 
    fn scopes(&self) -> Vec<&str>;

    async fn refresh(&mut self) -> Result<(), Error>;
}

pub trait Scope { }

pub struct Enabled;

pub struct Disabled;

impl Scope for Enabled { }

impl Scope for Disabled { }

/// Builder trait for building an oauth instance
pub trait OAuthBuilder: Sized + Clone {
    type ImplicitOAuth<'a>: OAuth + Sync + Send;

    fn with_secret(self, client_id: String, secret: String, redirect_uri: String) -> WithSecret<Self> {
        WithSecret { client_id, secret, redirect_uri, inner: self, url: "https://staging.worldcubeassociation.org/oauth/token".to_owned() }
    }

    fn with_manage_competition_scope(self) -> WithManageCompetition<Self> {
        WithManageCompetition(self)
    }

    fn scopes(&self) -> Vec<&str>;

    fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth<'static> {
        self.authenticate_implicit_with_client(access_token, &CLIENT)
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a>;
}

#[async_trait]
pub trait OAuthBuilderWithSecret: Sized + OAuthBuilder + Clone {
    type ExplicitOAuth<'a>: OAuth + Refreshable + Sync + Send;
    
    async fn authenticate_explicit(self, access_code: String) -> Result<Self::ExplicitOAuth<'static>, Error> {
        self.authenticate_explicit_with_client(access_code, &CLIENT).await
    }

    async fn authenticate_explicit_with_client<'a>(self, access_code: String, client: &'a Client) -> Result<Self::ExplicitOAuth<'a>, Error>;
}

pub trait OAuthManageCompetitions { }

impl<T> OAuthManageCompetitions for T where T: OAuth<ManageCompetitions = Enabled> { }

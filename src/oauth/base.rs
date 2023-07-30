use super::*;
use async_trait::async_trait;
use reqwest::{Method, RequestBuilder};

#[derive(Clone)]
pub struct OAuthBuilder(pub(super) &'static str);

impl OAuthBuilder {
    pub fn new() -> OAuthBuilder {
        OAuthBuilder("https://www.worldcubeassociation.org/api/v0/")
    }

    pub fn staging() -> OAuthBuilder {
        OAuthBuilder("https://staging.worldcubeassociation.org/api/v0/")
    }
}

impl ClientBuilder for OAuthBuilder {
    type ImplicitClient<'a> = ImplicitClient<'a>;

    fn scopes(&self) -> Vec<&str> {
        vec![]
    }

    fn authenticate_implicit_with_client<'a>(
        self, access_token: String, client: &'a reqwest::Client,
    ) -> Self::ImplicitClient<'a> {
        ImplicitClient {
            access_token,
            prefix: self.0.to_owned(),
            client,
        }
    }
}

pub struct ImplicitClient<'a> {
    access_token: String,
    prefix: String,
    client: &'a reqwest::Client,
}

#[async_trait]
impl Client for ImplicitClient<'_> {
    type Email = Disabled;

    type ManageCompetitions = Disabled;

    type DateOfBirth = Disabled;

    type Public = Disabled;

    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn request_builder(&self, suffix: &str, method: Method) -> RequestBuilder {
        let url = format!("{}{}", self.prefix, suffix);

        self.client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.access_token))
    }
}

impl SetAccessToken for ImplicitClient<'_> {
    fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }
}

pub struct PublicClient<'a> {
    client: &'a reqwest::Client,
    prefix: String,
}

impl PublicClient<'_> {
    pub fn new() -> PublicClient<'static> {
        PublicClient {
            client: &CLIENT,
            prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned(),
        }
    }

    pub fn new_with_client(client: &reqwest::Client) -> PublicClient<'_> {
        PublicClient {
            client,
            prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned(),
        }
    }

    pub fn staging() -> PublicClient<'static> {
        PublicClient {
            client: &CLIENT,
            prefix: "https://staging.worldcubeassociation.org/api/v0/".to_owned(),
        }
    }

    pub fn staging_with_client(client: &reqwest::Client) -> PublicClient<'_> {
        PublicClient {
            client,
            prefix: "https://staging.worldcubeassociation.org/api/v0/".to_owned(),
        }
    }
}

#[async_trait]
impl Client for PublicClient<'_> {
    type Email = Disabled;

    type ManageCompetitions = Disabled;

    type DateOfBirth = Disabled;

    type Public = Disabled;

    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    
    fn request_builder(&self, suffix: &str, method: Method) -> RequestBuilder {
        let url = format!("{}{}", self.prefix, suffix);

        self.client
            .request(method, &url)
    }
}

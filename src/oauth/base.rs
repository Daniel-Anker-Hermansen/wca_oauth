use async_trait::async_trait;
use reqwest::Client;
use super::*;

#[derive(Clone)]
pub struct BaseOAuthBuilder(pub(super) &'static str);

impl BaseOAuthBuilder {
    pub fn new() -> BaseOAuthBuilder {
        BaseOAuthBuilder("https://www.worldcubeassociation.org/api/v0/")
    }

    pub fn staging() -> BaseOAuthBuilder {
        BaseOAuthBuilder("https://staging.worldcubeassociation.org/api/v0/")
    }
}

impl OAuthBuilder for BaseOAuthBuilder {
    type ImplicitOAuth<'a> = ImplicitOAuth<'a>;

    fn scopes(&self) -> Vec<&str> {
        vec![]
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a> {
        ImplicitOAuth {
            access_token,
            prefix: self.0.to_owned(),
            client,
        }
    }
}

pub struct ImplicitOAuth<'a> {
    access_token: String,
    prefix: String,
    client: &'a Client,
}

#[async_trait]
impl OAuth for ImplicitOAuth<'_> {
    type Email = Disabled;

    type ManageCompetitions = Disabled;

    type DateOfBirth = Disabled;

    type Public = Disabled;

    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn client(&self ) -> &Client {
        &self.client
    }

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.prefix, suffix);
        
        self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?
            .text()
            .await
    }
}

impl SetAccessToken for ImplicitOAuth<'_> {
    fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }
}

impl LoggedIn for ImplicitOAuth<'_> { }

pub struct PublicApi<'a> {
    client: &'a Client,
    prefix: String,
}

impl PublicApi<'_> {
    pub fn new() -> PublicApi<'static> {
        PublicApi { client: &CLIENT, prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned() }
    }

    pub fn new_with_client(client: &Client) -> PublicApi<'_> {
        PublicApi { client, prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned() }
    }
    
    pub fn staging() -> PublicApi<'static> {
        PublicApi { client: &CLIENT, prefix: "https://staging.worldcubeassociation.org/api/v0/".to_owned() }
    }
    
    pub fn staging_with_client(client: &Client) -> PublicApi<'_> {
        PublicApi { client, prefix: "https://staging.worldcubeassociation.org/api/v0/".to_owned() }
    }
}

#[async_trait]
impl OAuth for PublicApi<'_> {
    type Email = Disabled;

    type ManageCompetitions = Disabled;

    type DateOfBirth = Disabled;

    type Public = Disabled;

    fn prefix(&self) ->  &str {
        &self.prefix
    }

    fn client(&self) -> &Client {
        &self.client
    }

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.prefix, suffix);
        
        self.client
            .get(&url)
            .send()
            .await?
            .text()
            .await
    }
}


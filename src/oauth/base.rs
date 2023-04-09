use async_trait::async_trait;
use reqwest::Client;
use super::*;

pub struct BaseOAuthBuilder;

impl OAuthBuilder for BaseOAuthBuilder {
    type ImplicitOAuth<'a> = ImplicitOAuth<'a>;

    fn scopes(&self) -> Vec<&str> {
        vec![]
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a> {
        ImplicitOAuth {
            access_token,
            prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned(),
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

    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
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

impl LoggedIn for ImplicitOAuth<'_> { }

pub struct PublicApi {
    client: Client,
    prefix: String,
}

impl PublicApi {
    pub fn new() -> PublicApi {
        PublicApi { client: Client::new(), prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned() }
    }
    
    pub fn staging() -> PublicApi {
        PublicApi { client: Client::new(), prefix: "https://staging.worldcubeassociation.org/api/v0/".to_owned() }
    }
}

#[async_trait]
impl OAuth for PublicApi {
    type Email = Disabled;

    type ManageCompetitions = Disabled;

    type DateOfBirth = Disabled;

    fn prefix(&self) ->  &str {
        &self.prefix
    }

    fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
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


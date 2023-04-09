use std::collections::HashSet;

use super::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct ExplicitOauth<T> {
    client_id: String,
    secret: String,
    redirect_uri: String,
    refresh_token: String,
    scope: String,
    inner: T,
}

#[async_trait]
impl<T> OAuth for ExplicitOauth<T> where T: OAuth + Send + Sync {
    type Email = T::Email;

    type ManageCompetitions = T::ManageCompetitions;

    type DateOfBirth = T::DateOfBirth;

    fn prefix(&self) ->  &str {
        self.inner.prefix()
    }

    fn set_prefix(&mut self, prefix: String) {
        self.inner.set_prefix(prefix);
    }

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error> {
        self.inner.custom_route(suffix).await
    }
}

impl<T> LoggedIn for ExplicitOauth<T> where T: OAuth + Send + Sync { }

#[async_trait]
impl<T> Refreshable for ExplicitOauth<T> where T: OAuth + Send + Sync {
    fn scopes(&self) -> Vec<&str> {
        self.scope.split_whitespace().collect()
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        todo!()
    }
}


pub struct WithSecret<T> {
    pub(super) client_id: String,
    pub(super) secret: String,
    pub(super) redirect_uri: String,
    pub(super) inner: T,
    pub(super) url: String,
}

impl<T> OAuthBuilder for WithSecret<T> where T: OAuthBuilder {
    type ImplicitOAuth<'a> = T::ImplicitOAuth<'a>;

    fn scopes(&self) -> Vec<&str> {
        self.inner.scopes()
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a> {
        self.inner.authenticate_implicit_with_client(access_token, client)
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for WithSecret<T> where T: OAuthBuilder + Send + Sync {
    type ExplicitOAuth<'a> = ExplicitOauth<T::ImplicitOAuth<'a>>;

    fn set_url(&mut self, url: String) {
        self.url = url;
    }

    async fn authenticate_explicit_with_client<'a>(self, access_code: String, client: &'a Client) -> Result<Self::ExplicitOAuth<'a>, Error> {
        let auth_response = get_auth_response(&self, &access_code).await?;

        let access_token = auth_response.access_token;

        let inner = self.inner.authenticate_implicit_with_client(access_token, client);
        Ok(ExplicitOauth {
            client_id: self.client_id,
            secret: self.secret,
            redirect_uri: self.redirect_uri,
            inner,
            refresh_token: auth_response.refresh_token,
            scope: auth_response.scope,
        })
    }
}

async fn get_auth_response<'a, T>(builder: &WithSecret<T>, access_code: &str) -> Result<AuthResponse, Error> where WithSecret<T>: OAuthBuilder {
    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", &builder.client_id),
        ("client_secret", &builder.secret),
        ("redirect_uri", &builder.redirect_uri),
        ("code", access_code.trim()),
    ];

    let response = Client::new()
        .post(&builder.url)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let auth_response = serde_json::from_str::<AuthResponse>(&response)
        .map_err(|_| serde_json::from_str::<ApiError>(&response)
            .map(|e| Error::from(e))
            .unwrap_or_else(|_| Error::from(format!("Error in requring authentification. The error did not conform to the expected format. Received following: {response}"))))?;

    let required_scopes: HashSet<_> = builder.scopes().into_iter().collect();
    let obtained_scopes: HashSet<_> = auth_response.scope.split_whitespace().collect();
    let mut difference_scopes = required_scopes.difference(&obtained_scopes);

    if let Some(scope) = difference_scopes.next() {
        return Err(Error::MissingScope(scope.to_string()));
    }

    Ok(auth_response)        
}    

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    //created_at: i64,
    //expires_in: i64,
    refresh_token: String,
    scope: String,
    //token_type: String,
}

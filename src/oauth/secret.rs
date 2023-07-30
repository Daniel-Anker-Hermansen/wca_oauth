use std::collections::HashSet;

use super::*;
use async_trait::async_trait;
use reqwest::{Method, RequestBuilder};
use serde::Deserialize;

pub struct ExplicitOauth<T> {
    client_id: String,
    secret: String,
    redirect_uri: String,
    refresh_token: String,
    scope: String,
    url: String,
    inner: T,
}

#[async_trait]
impl<T> Client for ExplicitOauth<T>
where T: Client + Send + Sync
{
    type Email = T::Email;

    type ManageCompetitions = T::ManageCompetitions;

    type DateOfBirth = T::DateOfBirth;

    type Public = T::Public;

    fn prefix(&self) -> &str {
        self.inner.prefix()
    }

    fn client(&self) -> &reqwest::Client {
        self.inner.client()
    }
    
    fn request_builder(&self, suffix: &str, method: Method) -> RequestBuilder {
        self.inner.request_builder(suffix, method)
    }
}

impl<T> SetAccessToken for ExplicitOauth<T>
where T: SetAccessToken
{
    fn set_access_token(&mut self, access_token: String) {
        self.inner
            .set_access_token(access_token);
    }
}

#[async_trait]
impl<T> RefreshableClient for ExplicitOauth<T>
where T: Client + SetAccessToken + Send + Sync
{
    fn scopes(&self) -> Vec<&str> {
        self.scope.split_whitespace().collect()
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &self.client_id),
            ("client_secret", &self.secret),
            ("redirect_uri", &self.redirect_uri),
            ("refresh_token", &self.refresh_token),
        ];

        let response = self
            .inner
            .client()
            .post(&self.url)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;

        let auth_response = serde_json::from_str::<AuthResponse>(&response)
            .map_err(|_| serde_json::from_str::<ApiError>(&response)
                .map(|e| Error::from(e))
                .unwrap_or_else(|_| Error::from(format!("Error in requring authentification. The error did not conform to the expected format. Received following: {response}"))))?;

        self.refresh_token = auth_response.refresh_token;
        self.set_access_token(auth_response.access_token);
        Ok(())
    }
}

#[derive(Clone)]
pub struct WithSecret<T> {
    pub(super) client_id: String,
    pub(super) secret: String,
    pub(super) redirect_uri: String,
    pub(super) inner: T,
    pub(super) url: String,
}

impl<T> ClientBuilder for WithSecret<T>
where T: ClientBuilder
{
    type ImplicitClient<'a> = T::ImplicitClient<'a>;

    fn scopes(&self) -> Vec<&str> {
        self.inner.scopes()
    }

    fn authenticate_implicit_with_client<'a>(
        self, access_token: String, client: &'a reqwest::Client,
    ) -> Self::ImplicitClient<'a> {
        self.inner
            .authenticate_implicit_with_client(access_token, client)
    }
}

#[async_trait]
impl<T> ClientWithSecretBuilder for WithSecret<T>
where
    T: ClientBuilder + Send + Sync,
    for<'a> <T as ClientBuilder>::ImplicitClient<'a>: SetAccessToken,
{
    type ExplicitClient<'a> = ExplicitOauth<T::ImplicitClient<'a>>;

    async fn authenticate_explicit_with_client<'a>(
        self, access_code: String, client: &'a reqwest::Client,
    ) -> Result<Self::ExplicitClient<'a>, Error> {
        let auth_response = get_auth_response(&self, client, &access_code).await?;

        let access_token = auth_response.access_token;

        let inner = self
            .inner
            .clone()
            .authenticate_implicit_with_client(access_token, client);
        Ok(ExplicitOauth {
            client_id: self.client_id,
            secret: self.secret,
            redirect_uri: self.redirect_uri,
            inner,
            url: self.url,
            refresh_token: auth_response.refresh_token,
            scope: auth_response.scope,
        })
    }
}

async fn get_auth_response<'a, T>(
    builder: &WithSecret<T>, client: &reqwest::Client, access_code: &str,
) -> Result<AuthResponse, Error>
where WithSecret<T>: ClientBuilder {
    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", &builder.client_id),
        ("client_secret", &builder.secret),
        ("redirect_uri", &builder.redirect_uri),
        ("code", access_code.trim()),
    ];

    let response = client
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
    let obtained_scopes: HashSet<_> = auth_response
        .scope
        .split_whitespace()
        .collect();
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

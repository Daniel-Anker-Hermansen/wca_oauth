use async_trait::async_trait;
use super::*;

#[derive(Clone)]
pub struct WithManageCompetition<T>(pub(super) T);

impl<T> OAuthBuilder for WithManageCompetition<T> where T: OAuthBuilder {
    type ImplicitOAuth<'a> = ManageCompetitionsScope<T::ImplicitOAuth<'a>>;

    fn scopes(&self) -> Vec< &str>  {
        let mut result = self.0.scopes();
        result.push("manage_competitions");
        result
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a> {
        ManageCompetitionsScope(self.0.authenticate_implicit_with_client(access_token, client))
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for WithManageCompetition<T> where T: OAuthBuilderWithSecret + Send {
    type ExplicitOAuth<'a> = ManageCompetitionsScope<T::ExplicitOAuth<'a>>;

    async fn authenticate_explicit_with_client<'a>(self, access_code: String, client: &'a Client) -> Result<Self::ExplicitOAuth<'a>, Error> {
        match self.0.authenticate_explicit_with_client(access_code, client).await {
            Ok(inner) => if inner.scopes().contains(&"manage_competitions") {
                    Ok(ManageCompetitionsScope(inner))
                }
                else {
                    Err(Error::MissingScope("manage_competitions".to_owned()))
                },
            Err(err) => Err(err),
        }
    }
}

pub struct ManageCompetitionsScope<T>(T);

#[async_trait]
impl<T> OAuth for ManageCompetitionsScope<T> where T: OAuth + Sync {
    type Email = T::Email;

    type ManageCompetitions = Enabled;

    type DateOfBirth = T::DateOfBirth;

    fn prefix(&self) -> &str {
        self.0.prefix()
    }

    fn client(&self) -> &Client {
        &self.0.client()
    }

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error> {
        let result = self.0.custom_route(suffix);
        result.await
    }
}

impl<T> SetAccessToken for ManageCompetitionsScope<T> where T: SetAccessToken {
    fn set_access_token(&mut self, access_token: String) {
        self.0.set_access_token(access_token);
    }
}

impl<T> LoggedIn for ManageCompetitionsScope<T> where T: OAuth + Send + Sync { }

#[async_trait]
impl<T> Refreshable for ManageCompetitionsScope<T> where T: Refreshable + Send {
    fn scopes(&self) -> Vec<&str> {
        self.0.scopes()
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        self.0.refresh().await
    }
}

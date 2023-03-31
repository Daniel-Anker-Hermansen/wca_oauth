use async_trait::async_trait;
use super::*;

pub struct WithManageCompetition<T>(pub(super) T);

impl<T> OAuthBuilder for WithManageCompetition<T> where T: OAuthBuilder {
    type ImplicitOAuth = ManageCompetitionsScope<T::ImplicitOAuth>;

    fn scopes(&self) -> Vec< &str>  {
        let mut result = self.0.scopes();
        result.push("manage_competitions");
        result
    }

    fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth {
        ManageCompetitionsScope(self.0.authenticate_implicit(access_token))
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for WithManageCompetition<T> where T: OAuthBuilderWithSecret + Send {
    type ExplicitOAuth = ManageCompetitionsScope<T::ExplicitOAuth>;

    fn set_url(&mut self, url: String) {
        self.0.set_url(url);
    }

    async fn authenticate_explicit(self, access_code: String) -> Result<Self::ExplicitOAuth, Error> {
        match self.0.authenticate_explicit(access_code).await {
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

    fn set_prefix(&mut self, prefix: String) {
        self.0.set_prefix(prefix)
    }

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error> {
        let result = self.0.custom_route(suffix);
        result.await
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

use async_trait::async_trait;
use super::*;

#[derive(Clone)]
pub struct WithDob<T>(pub(super) T);

impl<T> OAuthBuilder for WithDob<T> where T: OAuthBuilder {
    type ImplicitOAuth<'a> = DobScope<T::ImplicitOAuth<'a>>;

    fn scopes(&self) -> Vec< &str>  {
        let mut result = self.0.scopes();
        result.push("dob");
        result
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a> {
        DobScope(self.0.authenticate_implicit_with_client(access_token, client))
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for WithDob<T> where T: OAuthBuilderWithSecret + Send {
    type ExplicitOAuth<'a> = DobScope<T::ExplicitOAuth<'a>>;

    async fn authenticate_explicit_with_client<'a>(self, access_code: String, client: &'a Client) -> Result<Self::ExplicitOAuth<'a>, Error> {
        let inner = self.0.authenticate_explicit_with_client(access_code, client).await?;
        check_scope(inner, "dob").map(DobScope)
    }
}

pub struct DobScope<T>(T);

#[async_trait]
impl<T> OAuth for DobScope<T> where T: OAuth + Sync {
    type Email = T::Email;

    type ManageCompetitions = T::ManageCompetitions;

    type DateOfBirth = Enabled;

    type Public = T::Public;

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

impl<T> SetAccessToken for DobScope<T> where T: SetAccessToken {
    fn set_access_token(&mut self, access_token: String) {
        self.0.set_access_token(access_token);
    }
}

impl<T> LoggedIn for DobScope<T> where T: OAuth + Send + Sync { }

#[async_trait]
impl<T> Refreshable for DobScope<T> where T: Refreshable + Send {
    fn scopes(&self) -> Vec<&str> {
        self.0.scopes()
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        self.0.refresh().await
    }
}

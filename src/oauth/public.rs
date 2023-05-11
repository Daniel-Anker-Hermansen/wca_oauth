use async_trait::async_trait;
use super::*;

#[derive(Clone)]
pub struct WithPublic<T>(pub(super) T);

impl<T> OAuthBuilder for WithPublic<T> where T: OAuthBuilder {
    type ImplicitOAuth<'a> = PublicScope<T::ImplicitOAuth<'a>>;

    fn scopes(&self) -> Vec<&str> {
        let mut vec = self.0.scopes();
        vec.push("public");
        vec
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a> {
        PublicScope(self.0.authenticate_implicit_with_client(access_token, client))
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for WithPublic<T> where T: OAuthBuilderWithSecret + Send {
    type ExplicitOAuth<'a> = PublicScope<T::ExplicitOAuth<'a>>;

    async fn authenticate_explicit_with_client<'a>(self, access_code: String, client: &'a Client) -> Result<Self::ExplicitOAuth<'a>, Error> {
        let inner = self.0.authenticate_explicit_with_client(access_code, client).await?;
        check_scope(inner, "public").map(PublicScope)
    }
}

pub struct PublicScope<T>(T);

#[async_trait]
impl<T> OAuth for PublicScope<T> where T: OAuth + Sync {
    type Email = T::Email;

    type ManageCompetitions = T::ManageCompetitions;

    type DateOfBirth = T::DateOfBirth;

    type Public = Enabled;

    fn prefix(&self) ->  &str {
        self.0.prefix()
    }

    fn client(&self) ->  &Client {
        &self.0.client()
    }

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error> {
        let result = self.0.custom_route(suffix);
        result.await
    }
}

impl<T> SetAccessToken for PublicScope<T> where T: SetAccessToken {
    fn set_access_token(&mut self, access_token: String) {
        self.0.set_access_token(access_token)
    }
}

impl<T> LoggedIn for PublicScope<T> where T: OAuth + Send + Sync { }

#[async_trait]
impl<T> Refreshable for PublicScope<T> where T: Refreshable + Send {
    fn scopes(&self) -> Vec<&str> {
        self.0.scopes()
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        self.0.refresh().await
    }
}

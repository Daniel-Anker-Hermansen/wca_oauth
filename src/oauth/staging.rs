use async_trait::async_trait;

use super::*;

pub struct StagingBuilder<T>(pub(super) T);

impl<T> OAuthBuilder for StagingBuilder<T> where T: OAuthBuilder {
    type ImplicitOAuth<'a> = T::ImplicitOAuth<'a>;

    fn scopes(&self) -> Vec<&str> {
        self.0.scopes()
    }

    fn authenticate_implicit_with_client<'a>(self, access_token: String, client: &'a Client) -> Self::ImplicitOAuth<'a> {
        let mut oauth = self.0.authenticate_implicit_with_client(access_token, client);
        oauth.set_prefix("https://staging.worldcubeassociation.org/api/v0/".to_owned());
        oauth
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for StagingBuilder<T> where T: OAuthBuilderWithSecret + Sync + Send {
    type ExplicitOAuth<'a> = T::ExplicitOAuth<'a>;

    fn set_url(&mut self, url: String) {
        self.0.set_url(url);
    }

    async fn authenticate_explicit_with_client<'a>(mut self, access_code: String, client: &'a Client) -> Result<Self::ExplicitOAuth<'a>, Error> {
        self.0.set_url("https://staging.worldcubeassociation.org/oauth/token".to_owned());
        let mut oauth = self.0.authenticate_explicit_with_client(access_code, client).await?;
        oauth.set_prefix("https://staging.worldcubeassociation.org/api/v0/".to_owned());
        Ok(oauth)
    }
}

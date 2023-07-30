use std::marker::PhantomData;

use async_trait::async_trait;

use crate::{Client, ClientBuilder, ClientWithSecretBuilder, Enabled, Error, RefreshableClient};

use super::{check_scope, SetAccessToken};

use reqwest::{Method, RequestBuilder};

pub trait Scope: Clone {
    const NAME: &'static str;
}

#[derive(Clone)]
pub struct DateOfBirth;
#[derive(Clone)]
pub struct Email;
#[derive(Clone)]
pub struct ManageCompetititons;
#[derive(Clone)]
pub struct Public;

impl Scope for DateOfBirth {
    const NAME: &'static str = "dob";
}

impl Scope for Email {
    const NAME: &'static str = "email";
}

impl Scope for ManageCompetititons {
    const NAME: &'static str = "manage_competitions";
}

impl Scope for Public {
    const NAME: &'static str = "public";
}

#[derive(Clone)]
pub struct ScopedClientBuilder<S, B> {
    inner: B,
    _phantom: PhantomData<S>,
}

impl<S, B> ScopedClientBuilder<S, B> {
    pub fn new(builder: B) -> ScopedClientBuilder<S, B> {
        ScopedClientBuilder {
            inner: builder,
            _phantom: PhantomData,
        }
    }
}

impl<S, B> ClientBuilder for ScopedClientBuilder<S, B>
where
    S: Scope + Send + Sync,
    B: ClientBuilder,
    for<'a> ScopedClient<S, B::ImplicitClient<'a>>: Client,
{
    type ImplicitClient<'a> = ScopedClient<S, B::ImplicitClient<'a>>;

    fn scopes(&self) -> Vec<&str> {
        let mut scopes = self.inner.scopes();
        scopes.push(S::NAME);
        scopes
    }

    fn authenticate_implicit_with_client<'a>(
        self, access_token: String, client: &'a reqwest::Client,
    ) -> Self::ImplicitClient<'a> {
        let inner = self
            .inner
            .authenticate_implicit_with_client(access_token, client);
        ScopedClient {
            inner,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<S, B> ClientWithSecretBuilder for ScopedClientBuilder<S, B>
where
    S: Scope + Send + Sync,
    B: ClientWithSecretBuilder + Send,
    for<'a> ScopedClient<S, B::ImplicitClient<'a>>: Client,
    for<'a> ScopedClient<S, B::ExplicitClient<'a>>: Client,
{
    type ExplicitClient<'a> = ScopedClient<S, B::ExplicitClient<'a>>;

    async fn authenticate_explicit_with_client<'a>(
        self, access_code: String, client: &'a reqwest::Client,
    ) -> Result<Self::ExplicitClient<'a>, Error> {
        let inner = self
            .inner
            .authenticate_explicit_with_client(access_code, client)
            .await?;
        check_scope(inner, S::NAME).map(|inner| ScopedClient {
            inner,
            _phantom: PhantomData,
        })
    }
}

pub struct ScopedClient<S, C> {
    inner: C,
    _phantom: PhantomData<S>,
}

macro_rules! oauth_impl {
    ($scope:ty, $email:ty, $manage_competitions:ty, $date_of_birth:ty, $public:ty) => {
        #[async_trait]
        impl<C> Client for ScopedClient<$scope, C>
        where C: Client + Sync
        {
            type Email = $email;

            type ManageCompetitions = $manage_competitions;

            type DateOfBirth = $date_of_birth;

            type Public = $public;

            fn prefix(&self) -> &str {
                self.inner.prefix()
            }

            fn client(&self) -> &reqwest::Client {
                &self.inner.client()
            }
    
            fn request_builder(&self, suffix: &str, method: Method) -> RequestBuilder {
                self.inner.request_builder(suffix, method)
            }
        }
    };
}

oauth_impl!(DateOfBirth, C::Email, C::ManageCompetitions, Enabled, C::Public);
oauth_impl!(Email, Enabled, C::ManageCompetitions, C::DateOfBirth, C::Public);
oauth_impl!(ManageCompetititons, C::Email, Enabled, C::DateOfBirth, C::Public);
oauth_impl!(Public, C::Email, C::ManageCompetitions, C::DateOfBirth, Enabled);

#[async_trait]
impl<S, C> RefreshableClient for ScopedClient<S, C>
where
    S: Scope + Send,
    C: RefreshableClient + Send,
{
    fn scopes(&self) -> Vec<&str> {
        self.inner.scopes()
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        self.inner.refresh().await
    }
}

impl<S, C> SetAccessToken for ScopedClient<S, C>
where C: SetAccessToken
{
    fn set_access_token(&mut self, access_token: String) {
        self.inner
            .set_access_token(access_token);
    }
}

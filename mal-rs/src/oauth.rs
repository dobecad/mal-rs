use crate::OAUTH_URL;
use oauth2::basic::BasicClient;
use oauth2::http::Uri;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RequestTokenError, Scope, TokenResponse, StandardTokenResponse, AccessToken, RefreshToken,
};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::marker::PhantomData;
use std::time::Duration;
use url::Url;

use std::fmt;

#[derive(Debug)]
pub struct OauthResponseError {
    pub message: String,
}

impl Error for OauthResponseError {}

impl fmt::Display for OauthResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl OauthResponseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug)]
pub struct Unauthenticated;

#[derive(Debug)]
pub struct Authenticated;

#[derive(Debug)]
pub struct OauthClient<State = Unauthenticated> {
    client: BasicClient,
    csrf: CsrfToken,
    pkce_verifier: PkceCodeVerifier,
    state: PhantomData<State>,
    access_token: AccessToken,
    refresh_token: RefreshToken,
    expires_in: Duration,
}

impl OauthClient<Unauthenticated> {
    pub fn new() -> Self {
        let client_id =
            env::var("CLIENT_ID".to_string()).expect("Missing CLIENT_ID environment variable");
        let client_secret = env::var("CLIENT_SECRET".to_string())
            .expect("Missing CLIENT_SECRET environment variable");
        let redirect_url = env::var("REDIRECT_URL".to_string())
            .expect("Missing REDIRECT_URL environment variable");

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(OAUTH_URL.to_string()).unwrap(),
            None,
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Malformed REDIRECT_URL"));

        Self {
            client,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            csrf: CsrfToken::new(String::from("")),
            state: PhantomData::<Unauthenticated>,
            access_token: AccessToken::new("".to_string()),
            refresh_token: RefreshToken::new("".to_string()),
            expires_in: Duration::new(0, 0)
        }
    }

    pub fn generate_readonly_auth_url(&mut self) -> String {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_plain();

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge)
            .url();

        self.csrf = csrf_token;
        self.pkce_verifier = pkce_verifier;

        auth_url.to_string()
    }

    pub fn generate_write_auth_url(&mut self) -> String {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_plain();

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("write:users".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        self.csrf = csrf_token;
        self.pkce_verifier = pkce_verifier;

        auth_url.to_string()
    }

    pub async fn authenticate(
        self,
        authorization_response: Uri,
    ) -> Result<OauthClient<Authenticated>, Box<dyn Error>> {
        let redirect_response = RedirectResponse::new(&authorization_response)?;
        if redirect_response.state != *self.csrf.secret() {
            return Err(Box::new(OauthResponseError::new("State does not match".to_string())));
        }

        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(redirect_response.code))
            .set_pkce_verifier(self.pkce_verifier)
            .request_async(async_http_client)
            .await?;

        Ok(OauthClient::<Authenticated> {
            client: self.client,
            csrf: self.csrf,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            state: PhantomData::<Authenticated>,
            access_token: token_result.access_token().to_owned(),
            refresh_token: token_result.refresh_token().unwrap().to_owned(),
            expires_in: token_result.expires_in().unwrap().to_owned()
        })
    }
}

impl OauthClient<Authenticated> {
    pub fn get_access_token(&self) -> &String {
        &self.access_token.secret()
    }

    pub fn get_refresh_token(&self) -> &String {
        &self.refresh_token.secret()
    }

    pub fn get_expires_in(&self) -> &Duration {
        &self.expires_in
    }
}

#[derive(Debug, Deserialize)]
struct RedirectResponse {
    code: String,
    state: String,
}

impl RedirectResponse {
    fn new(uri: &Uri) -> Result<RedirectResponse, OauthResponseError> {
        let query_params: Option<Self> = uri.query().map(|query| {
            serde_urlencoded::from_str(query).expect("Failed to get code and state from response.")
        });

        match query_params {
            Some(q) => Ok(q),
            None => Err(OauthResponseError::new(
                "Failed to get code and state from authorization redirect".to_string(),
            )),
        }
    }
}
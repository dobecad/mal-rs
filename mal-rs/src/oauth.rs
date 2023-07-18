//! Module for working through MAL OAuth2 flow

use crate::{OAUTH_TOKEN_URL, OAUTH_URL};
use oauth2::basic::BasicClient;
use oauth2::http::Uri;
use oauth2::reqwest::async_http_client;
pub use oauth2::ClientId;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::marker::PhantomData;
use std::time::{Duration, SystemTime};
use url::Url;

use std::fmt;

// Expiration date for access tokens is one month
// We use 28 days in seconds to be safe
const EXPIRATION_IN_SECONDS: u64 = 2419200;

#[derive(Debug)]
pub struct OauthError {
    pub message: String,
}

impl Error for OauthError {}

impl fmt::Display for OauthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl OauthError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, Clone)]
pub struct MalClientId(pub ClientId);

impl MalClientId {
    /// Create a [MalClientId] by passing in your ClientId as a string
    ///
    /// Useful if you want to control how your program fetches your MAL `CLIENT_ID`
    pub fn new(id: String) -> Self {
        let client_id = ClientId::new(id);
        Self(client_id)
    }

    /// Try to load your MAL ClientId from the environment variable `CLIENT_ID`
    pub fn from_env() -> Result<Self, OauthError> {
        let client_id = env::var("CLIENT_ID")
            .map_err(|err| OauthError::new(format!("Failed to load CLIENT_ID: {}", err)))?;
        Ok(Self(ClientId::new(client_id)))
    }
}

/// State struct for separating an Authenticated and Unauthenticated OAuthClient
#[derive(Debug)]
pub struct Unauthenticated;

/// State struct for separating an Authenticated and Unauthenticated OAuthClient
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
    expires_at: u64,
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
            Some(TokenUrl::new(OAUTH_TOKEN_URL.to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Malformed REDIRECT_URL"));

        Self {
            client,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            csrf: CsrfToken::new(String::from("")),
            state: PhantomData::<Unauthenticated>,
            access_token: AccessToken::new("".to_string()),
            refresh_token: RefreshToken::new("".to_string()),
            expires_at: Duration::new(0, 0).as_secs(),
        }
    }

    pub fn generate_auth_url(&mut self) -> String {
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

    pub async fn authenticate(
        self,
        authorization_response: RedirectResponse,
    ) -> Result<OauthClient<Authenticated>, Box<dyn Error>> {
        if authorization_response.state != *self.csrf.secret() {
            return Err(Box::new(OauthError::new(
                "State does not match".to_string(),
            )));
        }

        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(authorization_response.code))
            .set_pkce_verifier(self.pkce_verifier)
            .request_async(async_http_client)
            .await?;

        let now = calculate_current_system_time();

        Ok(OauthClient::<Authenticated> {
            client: self.client,
            csrf: self.csrf,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            state: PhantomData::<Authenticated>,
            access_token: token_result.access_token().to_owned(),
            refresh_token: token_result.refresh_token().unwrap().to_owned(),
            expires_at: now + Duration::from_secs(EXPIRATION_IN_SECONDS).as_secs(),
        })
    }
}

impl OauthClient<Authenticated> {
    /// Get the access token for the OauthClient
    pub(crate) fn get_access_token(&self) -> &AccessToken {
        &self.access_token
    }

    /// Get the access token secret value
    pub fn get_access_token_secret(&self) -> &String {
        &self.access_token.secret()
    }

    /// Get the refresh token secret value
    pub fn get_refresh_token_secret(&self) -> &String {
        &self.refresh_token.secret()
    }

    /// Get the time at which the token will expire
    ///
    /// The time is represented as number of seconds since the Unix Epoch
    pub fn get_expires_at(&self) -> &u64 {
        &self.expires_at
    }

    /// Refresh the access token using the refresh token
    pub async fn refresh(self) -> Result<Self, Box<dyn Error>> {
        let refresh_result = self
            .client
            .exchange_refresh_token(&self.refresh_token)
            .request_async(async_http_client)
            .await?;

        let now = calculate_current_system_time();

        Ok(OauthClient::<Authenticated> {
            client: self.client,
            csrf: self.csrf,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            state: PhantomData::<Authenticated>,
            access_token: refresh_result.access_token().to_owned(),
            refresh_token: refresh_result.refresh_token().unwrap().to_owned(),
            expires_at: now + Duration::from_secs(EXPIRATION_IN_SECONDS).as_secs(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct RedirectResponse {
    code: String,
    state: String,
}

impl RedirectResponse {
    /// Create a RedirectResponse from the given OAuth2 redirect result
    pub fn new(uri: &Uri) -> Result<RedirectResponse, OauthError> {
        let query_params: Option<Self> = uri.query().map(|query| {
            serde_urlencoded::from_str(query).expect("Failed to get code and state from response.")
        });

        match query_params {
            Some(q) => Ok(q),
            None => Err(OauthError::new(
                "Failed to get code and state from authorization redirect".to_string(),
            )),
        }
    }
}

impl TryFrom<String> for RedirectResponse {
    type Error = OauthError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let query_string = value
            .parse::<Url>()
            .map_err(|err| OauthError::new(format!("Given string is not a valid URL: {}", err)))?;

        let query_params = query_string.query().ok_or_else(|| {
            OauthError::new("Failed to get code and state from redirect".to_string())
        })?;

        serde_urlencoded::from_str::<RedirectResponse>(&query_params)
            .map_err(|_| OauthError::new("Failed to get code and state from redirect".to_string()))
    }
}

fn calculate_current_system_time() -> u64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get current time")
        .as_secs();
    now
}

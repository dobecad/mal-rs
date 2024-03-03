//! Module for working through MAL OAuth2 flow

use crate::{OAUTH_TOKEN_URL, OAUTH_URL};
use oauth2::basic::BasicClient;
use oauth2::http::Uri;
use oauth2::reqwest::async_http_client;
use oauth2::ClientId;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::marker::PhantomData;
use std::path::Path;
use std::time::{Duration, SystemTime};
use std::{env, fs};
use thiserror::Error;
use toml;
use url::Url;

// Expiration date for access tokens is one month
// We use 28 days in seconds to be safe
const EXPIRATION_IN_SECONDS: u64 = 2419200;

const CONFIG_LOCATION: &'static str = ".mal/config.toml";

#[derive(Debug, Error)]
pub enum OauthError {
    #[error("missing environment variable")]
    MissingEnvVar,

    #[error("missing client id")]
    MissingClientId,

    #[error("missing client secret")]
    MissingClientSecret,

    #[error("missing redirect url")]
    MissingRedirectUrl,

    #[error("received state does not match")]
    StateMismatch,

    #[error("bad token response")]
    BadTokenResponse,

    #[error("invalid redirect url")]
    InvalidRedirectUrl,

    #[error("invalid redirect response")]
    InvalidRedirectResponse,

    #[error("missing access token")]
    MissingAccessToken,

    #[error("missing refresh token")]
    MissingRefreshToken,

    #[error("missing token expiration time")]
    MissingTokenExpiration,

    #[error("missing config")]
    MissingConfig,

    #[error("invalid config format")]
    InvalidConfigFormat,

    #[error("failed to create config")]
    ConfigCreationFailure,

    #[error("unable to fetch system time")]
    NoSystemTime,

    #[error("invalid expiration time")]
    InvalidExpirationTime,

    #[error("failed to refresh the authentication token")]
    FailedToRefreshToken,

    #[error("missing the code or state from response")]
    MissingCodeOrState,
}

/// If you only need to access public information on MAL that does
/// not require an Oauth access token, you can use the [MalClientId]
/// as your authorization client
#[derive(Debug)]
pub struct MalClientId(pub ClientId);

impl MalClientId {
    /// Create a [MalClientId] by passing in your ClientId as a string
    ///
    /// Useful if you want to control how your program fetches your MAL `MAL_CLIENT_ID`
    pub fn new<T: Into<String>>(id: T) -> Self {
        let client_id = ClientId::new(id.into());
        Self(client_id)
    }

    /// Try to load your MAL ClientId from the environment variable `MAL_CLIENT_ID`
    pub fn try_from_env() -> Result<Self, OauthError> {
        let client_id = OauthClient::load_client_id_from_env()?;
        Ok(Self(ClientId::new(client_id)))
    }
}

/// State struct for separating an Authenticated and Unauthenticated OAuthClient
#[derive(Debug)]
pub struct Unauthenticated;

/// State struct for separating an Authenticated and Unauthenticated OAuthClient
#[derive(Debug)]
pub struct Authenticated;

/// Client used to navigate and manage Oauth credentials with MAL
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
    /// Creates a new [OauthClient] for the PKCE flow
    pub fn new<T: Into<String>>(
        client_id: T,
        client_secret: Option<T>,
        redirect_url: T,
    ) -> Result<Self, OauthError> {
        let (client_id, redirect_url) = (client_id.into(), redirect_url.into());
        let client_secret = client_secret.map(|c| c.into());

        let client = Self::create_oauth2_client(client_id, client_secret, redirect_url)?;

        Ok(Self {
            client,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            csrf: CsrfToken::new(String::from("")),
            state: PhantomData::<Unauthenticated>,
            access_token: AccessToken::new("".to_string()),
            refresh_token: RefreshToken::new("".to_string()),
            expires_at: Duration::new(0, 0).as_secs(),
        })
    }

    fn create_oauth2_client(
        client_id: String,
        client_secret: Option<String>,
        redirect_url: String,
    ) -> Result<BasicClient, OauthError> {
        match client_secret {
            Some(c) => {
                let client = BasicClient::new(
                    ClientId::new(client_id),
                    Some(ClientSecret::new(c.into())),
                    AuthUrl::new(OAUTH_URL.to_string()).unwrap(),
                    Some(TokenUrl::new(OAUTH_TOKEN_URL.to_string()).unwrap()),
                )
                .set_redirect_uri(
                    RedirectUrl::new(redirect_url).map_err(|_| OauthError::InvalidRedirectUrl)?,
                )
                .set_auth_type(oauth2::AuthType::BasicAuth);
                Ok(client)
            }
            None => {
                let client = BasicClient::new(
                    ClientId::new(client_id),
                    None,
                    AuthUrl::new(OAUTH_URL.to_string()).unwrap(),
                    Some(TokenUrl::new(OAUTH_TOKEN_URL.to_string()).unwrap()),
                )
                .set_redirect_uri(
                    RedirectUrl::new(redirect_url).map_err(|_| OauthError::InvalidRedirectUrl)?,
                )
                .set_auth_type(oauth2::AuthType::RequestBody);
                Ok(client)
            }
        }
    }

    /// Generate an authorization URL for the user to navigate to,
    /// to begin the authorization process
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

    /// Try and authenticate the client to get an authenticated Oauth client back.
    pub async fn authenticate(
        self,
        authorization_response: RedirectResponse,
    ) -> Result<OauthClient<Authenticated>, OauthError> {
        if authorization_response.state != *self.csrf.secret() {
            return Err(OauthError::StateMismatch);
        }

        let code = AuthorizationCode::new(authorization_response.code);
        let token_result = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(self.pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|_| OauthError::BadTokenResponse)?;

        let now = calculate_current_system_time()?;

        Ok(OauthClient::<Authenticated> {
            client: self.client,
            csrf: self.csrf,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            state: PhantomData::<Authenticated>,
            access_token: token_result.access_token().to_owned(),
            refresh_token: token_result
                .refresh_token()
                .ok_or_else(|| OauthError::MissingRefreshToken)?
                .to_owned(),
            expires_at: now
                + token_result
                    .expires_in()
                    .unwrap_or(Duration::from_secs(EXPIRATION_IN_SECONDS))
                    .as_secs(),
        })
    }

    /// Load Oauth credentials from the environment
    ///
    /// `Note`: This is expected to work after saving the credentials from an
    /// authenticated OauthClient
    fn load_from_env() -> Result<OauthClient<Authenticated>, OauthError> {
        let (client_id, redirect_url) = (
            Self::load_client_id_from_env()?,
            Self::load_redirect_url_from_env()?,
        );
        let client_secret = Self::load_client_secret_from_env().ok();

        let client = Self::create_oauth2_client(client_id, client_secret, redirect_url)?;

        let access_token = Self::load_env_var("MAL_ACCESS_TOKEN")?;
        let refresh_token = Self::load_env_var("MAL_REFRESH_TOKEN")?;
        let expires_at = Self::load_env_var("MAL_TOKEN_EXPIRES_AT")
            .map_err(|_| OauthError::MissingTokenExpiration)?
            .parse::<u64>()
            .map_err(|_| OauthError::InvalidExpirationTime)?;

        Ok(OauthClient::<Authenticated> {
            client,
            csrf: CsrfToken::new(String::default()),
            pkce_verifier: PkceCodeVerifier::new(String::default()),
            state: PhantomData::<Authenticated>,
            access_token: AccessToken::new(access_token),
            refresh_token: RefreshToken::new(refresh_token),
            expires_at,
        })
    }

    /// Load Oauth credentials from the MAL config
    ///
    /// It is recommended to refresh the client after loading to ensure
    /// that all of the tokens are still valid
    pub fn load_from_config() -> Result<OauthClient<Authenticated>, OauthError> {
        if !Path::new(CONFIG_LOCATION).exists() {
            return Err(OauthError::MissingConfig);
        }

        let toml_content =
            fs::read_to_string(CONFIG_LOCATION).map_err(|_| OauthError::MissingConfig)?;
        let parsed_toml: MalCredentialsConfig =
            toml::from_str(&toml_content).map_err(|_| OauthError::InvalidConfigFormat)?;

        env::set_var("MAL_ACCESS_TOKEN", parsed_toml.mal_access_token.to_string());
        env::set_var(
            "MAL_REFRESH_TOKEN",
            parsed_toml.mal_refresh_token.to_string(),
        );
        env::set_var(
            "MAL_TOKEN_EXPIRES_AT",
            parsed_toml.mal_token_expires_at.to_string(),
        );
        Self::load_from_env()
    }

    /// Load an authenticated OauthClient by passing the necessary values
    ///
    /// It's recommended to refresh the client after to ensure that
    /// the given values are still valid credentials.
    ///
    /// `Note`: This method still relies on the `MAL_CLIENT_ID`, `MAL_CLIENT_SECRET`, and
    /// `MAL_REDIRECT_URL` environment variables being set
    pub fn load_from_values<T: Into<String>>(
        access_token: T,
        refresh_token: T,
        client_id: T,
        client_secret: Option<T>,
        redirect_url: T,
        expires_at: u64,
    ) -> Result<OauthClient<Authenticated>, OauthError> {
        let (access_token, refresh_token) = (access_token.into(), refresh_token.into());
        let (client_id, client_secret, redirect_url) = (
            client_id.into(),
            client_secret.map(|c| c.into()),
            redirect_url.into(),
        );

        let unix_epoch = SystemTime::UNIX_EPOCH
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| OauthError::NoSystemTime)?
            .as_secs();

        if expires_at < unix_epoch {
            return Err(OauthError::InvalidExpirationTime);
        }

        let client = Self::create_oauth2_client(client_id, client_secret, redirect_url)?;

        Ok(OauthClient::<Authenticated> {
            client,
            csrf: CsrfToken::new(String::default()),
            pkce_verifier: PkceCodeVerifier::new(String::default()),
            state: PhantomData::<Authenticated>,
            access_token: AccessToken::new(access_token),
            refresh_token: RefreshToken::new(refresh_token),
            expires_at,
        })
    }

    fn load_env_var(name: &str) -> Result<String, OauthError> {
        let result = env::var(name).map_err(|_| OauthError::MissingEnvVar)?;
        Ok(result)
    }

    /// Load the MAL_CLIENT_ID environment variable
    pub fn load_client_id_from_env() -> Result<String, OauthError> {
        let client_id =
            Self::load_env_var("MAL_CLIENT_ID").map_err(|_| OauthError::MissingClientId)?;
        Ok(client_id)
    }

    /// Load the MAL_CLIENT_SECRET environment variable
    pub fn load_client_secret_from_env() -> Result<String, OauthError> {
        let client_secret =
            Self::load_env_var("MAL_CLIENT_SECRET").map_err(|_| OauthError::MissingClientSecret)?;
        Ok(client_secret)
    }

    /// Load the MAL_REDIRECT_URL environment variable
    pub fn load_redirect_url_from_env() -> Result<String, OauthError> {
        let redirect_url =
            Self::load_env_var("MAL_REDIRECT_URL").map_err(|_| OauthError::MissingRedirectUrl)?;
        Ok(redirect_url)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MalCredentialsConfig {
    mal_access_token: String,
    mal_refresh_token: String,
    mal_token_expires_at: u64,
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

    /// Save the Oauth credentials to the config
    pub fn save_to_config(&self) -> Result<(), OauthError> {
        let config = MalCredentialsConfig {
            mal_access_token: self.access_token.secret().clone(),
            mal_refresh_token: self.refresh_token.secret().clone(),
            mal_token_expires_at: *self.get_expires_at(),
        };
        let toml = toml::to_string(&config).map_err(|_| OauthError::InvalidConfigFormat)?;

        if let Some(parent_dir) = Path::new(CONFIG_LOCATION).parent() {
            fs::create_dir_all(parent_dir).map_err(|_| OauthError::ConfigCreationFailure)?;
        }

        fs::write(CONFIG_LOCATION, toml).map_err(|_| OauthError::ConfigCreationFailure)?;
        Ok(())
    }

    /// Refresh the access token using the refresh token
    pub async fn refresh(self) -> Result<Self, Box<dyn Error>> {
        let refresh_result = self
            .client
            .exchange_refresh_token(&self.refresh_token)
            .request_async(async_http_client)
            .await
            .map_err(|_| OauthError::FailedToRefreshToken)?;

        let now = calculate_current_system_time()?;

        Ok(OauthClient::<Authenticated> {
            client: self.client,
            csrf: self.csrf,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            state: PhantomData::<Authenticated>,
            access_token: refresh_result.access_token().to_owned(),
            refresh_token: refresh_result.refresh_token().unwrap().to_owned(),
            expires_at: now
                + refresh_result
                    .expires_in()
                    .unwrap_or(Duration::from_secs(EXPIRATION_IN_SECONDS))
                    .as_secs(),
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
    ///
    /// Ultimately, this function just requires a reference to a Uri, that includes
    /// the `code` and `state` parameters
    pub fn new(uri: &Uri) -> Result<RedirectResponse, OauthError> {
        let query_params: Option<Self> = uri.query().map(|query| {
            serde_urlencoded::from_str(query).expect("Failed to get code and state from response.")
        });

        match query_params {
            Some(q) => Ok(q),
            None => Err(OauthError::InvalidRedirectResponse),
        }
    }
}

impl TryFrom<String> for RedirectResponse {
    type Error = OauthError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let query_string = value
            .parse::<Url>()
            .map_err(|_| OauthError::InvalidRedirectResponse)?;

        let query_params = query_string
            .query()
            .ok_or_else(|| OauthError::MissingCodeOrState)?;

        serde_urlencoded::from_str::<RedirectResponse>(&query_params)
            .map_err(|_| OauthError::MissingCodeOrState)
    }
}

fn calculate_current_system_time() -> Result<u64, OauthError> {
    let now = SystemTime::UNIX_EPOCH
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| OauthError::NoSystemTime)?
        .as_secs();
    Ok(now)
}

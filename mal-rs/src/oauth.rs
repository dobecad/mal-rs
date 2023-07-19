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
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::marker::PhantomData;
use std::path::Path;
use std::time::{Duration, SystemTime};
use std::{env, fs};
use toml;
use url::Url;

use std::fmt;

// Expiration date for access tokens is one month
// We use 28 days in seconds to be safe
const EXPIRATION_IN_SECONDS: u64 = 2419200;

const CONFIG_LOCATION: &'static str = ".mal/config.toml";

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

/// If you only need to access public information on MAL that does
/// not require an Oauth access token, you can use the [MalClientId]
/// as your authorization client
#[derive(Debug, Clone)]
pub struct MalClientId(pub ClientId);

impl MalClientId {
    /// Create a [MalClientId] by passing in your ClientId as a string
    ///
    /// Useful if you want to control how your program fetches your MAL `MAL_CLIENT_ID`
    pub fn new(id: String) -> Self {
        let client_id = ClientId::new(id);
        Self(client_id)
    }

    /// Try to load your MAL ClientId from the environment variable `MAL_CLIENT_ID`
    pub fn from_env() -> Result<Self, OauthError> {
        let client_id = env::var("MAL_CLIENT_ID")
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
    pub fn new() -> Result<Self, OauthError> {
        let client_id = env::var("MAL_CLIENT_ID".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_CLIENT_ID environment variable".to_string())
        })?;
        let client_secret = env::var("MAL_CLIENT_SECRET".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_CLIENT_SECRET environment variable".to_string())
        })?;
        let redirect_url = env::var("MAL_REDIRECT_URL".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_REDIRECT_URL environment variable".to_string())
        })?;

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(OAUTH_URL.to_string()).unwrap(),
            Some(TokenUrl::new(OAUTH_TOKEN_URL.to_string()).unwrap()),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url)
                .map_err(|err| OauthError::new(format!("Malformed REDIRECT_URL: {}", err)))?,
        );

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
            .await
            .map_err(|err| OauthError::new(format!("Failed to authenticate token: {}", err)))?;

        let now = calculate_current_system_time()?;

        Ok(OauthClient::<Authenticated> {
            client: self.client,
            csrf: self.csrf,
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            state: PhantomData::<Authenticated>,
            access_token: token_result.access_token().to_owned(),
            refresh_token: token_result
                .refresh_token()
                .expect("Missing refresh token")
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
        let client_id = env::var("MAL_CLIENT_ID".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_CLIENT_ID environment variable".to_string())
        })?;
        let client_secret = env::var("MAL_CLIENT_SECRET".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_CLIENT_SECRET environment variable".to_string())
        })?;
        let redirect_url = env::var("MAL_REDIRECT_URL".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_REDIRECT_URL environment variable".to_string())
        })?;

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(OAUTH_URL.to_string()).unwrap(),
            Some(TokenUrl::new(OAUTH_TOKEN_URL.to_string()).unwrap()),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url)
                .map_err(|e| OauthError::new(format!("Malformed REDIRECT_URL: {}", e)))?,
        );

        let access_token = env::var("MAL_ACCESS_TOKEN")
            .map_err(|_| OauthError::new("MAL_ACCESS_TOKEN is missing".to_string()))?;
        let refresh_token = env::var("MAL_REFRESH_TOKEN")
            .map_err(|_| OauthError::new("MAL_REFRESH_TOKEN is missing".to_string()))?;
        let expires_at = env::var("MAL_TOKEN_EXPIRES_AT")
            .map_err(|_| OauthError::new("MAL_TOKEN_EXPIRES_AT is missing".to_string()))?
            .parse::<u64>()
            .map_err(|_| OauthError::new("Failed to parse MAL_TOKEN_EXPIRES_AT".to_string()))?;

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
            return Err(OauthError::new(format!(
                "Failed to find config at {}",
                CONFIG_LOCATION
            )));
        }

        let toml_content = fs::read_to_string(CONFIG_LOCATION)
            .map_err(|err| OauthError::new(format!("Failed to load config: {}", err)))?;
        let parsed_toml: MalCredentialsConfig = toml::from_str(&toml_content)
            .map_err(|err| OauthError::new(format!("Failed to parse config: {}", err)))?;

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
    pub fn load_from_values(
        access_token: String,
        refresh_token: String,
        expires_at: u64,
    ) -> Result<OauthClient<Authenticated>, OauthError> {
        let client_id = env::var("MAL_CLIENT_ID".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_CLIENT_ID environment variable".to_string())
        })?;
        let client_secret = env::var("MAL_CLIENT_SECRET".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_CLIENT_SECRET environment variable".to_string())
        })?;
        let redirect_url = env::var("MAL_REDIRECT_URL".to_string()).map_err(|_| {
            OauthError::new("Missing MAL_REDIRECT_URL environment variable".to_string())
        })?;

        let unix_epoch = SystemTime::UNIX_EPOCH
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|err| OauthError::new(format!("Failed to get system time: {}", err)))?
            .as_secs();

        if expires_at < unix_epoch {
            return Err(OauthError::new(format!(
                "Invalid expires_at value. Must be greater than {}",
                unix_epoch
            )));
        }

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(OAUTH_URL.to_string()).unwrap(),
            Some(TokenUrl::new(OAUTH_TOKEN_URL.to_string()).unwrap()),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url)
                .map_err(|e| OauthError::new(format!("Malformed REDIRECT_URL: {}", e)))?,
        );

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
        let toml = toml::to_string(&config)
            .map_err(|err| OauthError::new(format!("Failed to turn config into toml: {}", err)))?;

        if let Some(parent_dir) = Path::new(CONFIG_LOCATION).parent() {
            fs::create_dir_all(parent_dir).map_err(|err| {
                OauthError::new(format!("Failed to create parent directory: {}", err))
            })?;
        }

        fs::write(CONFIG_LOCATION, toml)
            .map_err(|err| OauthError::new(format!("Failed to write to config: {}", err)))?;
        Ok(())
    }

    /// Refresh the access token using the refresh token
    pub async fn refresh(self) -> Result<Self, Box<dyn Error>> {
        let refresh_result = self
            .client
            .exchange_refresh_token(&self.refresh_token)
            .request_async(async_http_client)
            .await
            .map_err(|err| OauthError::new(format!("Failed to refresh token: {}", err)))?;

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

fn calculate_current_system_time() -> Result<u64, OauthError> {
    let now = SystemTime::UNIX_EPOCH
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|err| OauthError::new(format!("Failed to get system time: {}", err)))?
        .as_secs();
    Ok(now)
}

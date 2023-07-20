use oauth2::AccessToken;
use reqwest;
use serde::Serialize;

use crate::{
    oauth::{Authenticated, OauthClient},
    USER_URL,
};

use super::{error::UserApiError, requests::GetUserInformation, responses::User};

/// The UserApiClient provides functions for interacting with the various
/// `anime` and `user animelist` MAL API endpoints. A UserApiClient
/// can only be created from an [OauthClient].
///
/// # Example:
///
/// ```rust,ignore
/// use dotenvy;
/// use mal_rs::oauth::RedirectResponse;
/// use mal_rs::prelude::*;
/// use std::io;
///
/// #[tokio::main]
/// async fn main() {
///     dotenvy::dotenv().ok();
///
///     let mut oauth_client = OauthClient::new().unwrap();
///     println!("Visit this URL: {}\n", oauth_client.generate_auth_url());
///
///     println!("After authorizing, please enter the URL you were redirected to: ");
///     let mut input = String::new();
///     io::stdin()
///         .read_line(&mut input)
///         .expect("Failed to read user input");
///
///     let response = RedirectResponse::try_from(input).unwrap();
///
///     // Authentication process
///     let result = oauth_client.authenticate(response).await;
///     let authenticated_oauth_client = match result {
///         Ok(t) => {
///             println!("Got token: {:?}\n", t.get_access_token_secret());
///
///             let t = t.refresh().await.unwrap();
///             println!("Refreshed token: {:?}", t.get_access_token_secret());
///             t
///         }
///         Err(e) => panic!("Failed: {}", e),
///     };
///
///     // Using Oauth access token to interact with User API
///     let api_client = UserApiClient::from(&authenticated_oauth_client);
///     let fields = mal_rs::user_fields!(UserField::id, UserField::name, UserField::is_supporter);
///     let query = GetUserInformation::new(Some(&fields));
///     let response = api_client.get_my_user_information(&query).await.unwrap();
///     println!("Information about yourself: {:?}", response);
/// }
/// ```

pub struct UserApiClient {
    client: reqwest::Client,
    access_token: String,
}

impl From<&AccessToken> for UserApiClient {
    fn from(value: &AccessToken) -> Self {
        Self {
            client: reqwest::Client::new(),
            access_token: value.secret().clone(),
        }
    }
}

impl From<&OauthClient<Authenticated>> for UserApiClient {
    fn from(value: &OauthClient<Authenticated>) -> Self {
        UserApiClient {
            client: reqwest::Client::new(),
            access_token: value.get_access_token().secret().clone(),
        }
    }
}

impl UserApiClient {
    async fn get<T>(&self, query: &T) -> Result<String, UserApiError>
    where
        T: Serialize,
    {
        let response = self
            .client
            .get(format!("{}/@me", USER_URL))
            .bearer_auth(&self.access_token)
            .query(&query)
            .send()
            .await
            .map_err(|err| UserApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    /// Get information about the OAuth user
    ///
    /// Corresponds to the [Get my user information](https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_get) endpoint
    pub async fn get_my_user_information(
        &self,
        query: &GetUserInformation,
    ) -> Result<User, UserApiError> {
        let response = self.get(query).await?;
        let result: User = serde_json::from_str(response.as_str()).map_err(|err| {
            UserApiError::new(format!("Failed to parse AnimeList result: {}", err))
        })?;
        Ok(result)
    }
}

async fn handle_response(response: reqwest::Response) -> Result<String, UserApiError> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let content = response.text().await.map_err(|err| {
                UserApiError::new(format!("Failed to get content from response: {}", err))
            })?;
            Ok(content)
        }
        _ => Err(UserApiError::new(format!(
            "Did not recieve OK response: {}",
            response.status()
        ))),
    }
}

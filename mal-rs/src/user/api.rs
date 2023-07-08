use oauth2::AccessToken;
use reqwest;
use serde::Serialize;
use std::error::Error;

use crate::{
    oauth::{Authenticated, OauthClient},
    USER_URL,
};

use super::{error::UserApiError, requests::GetUserInformation, responses::User};

/// The UserApiClient provides functions for interacting with the various
/// `anime` and `user animelist` MAL API endpoints. A UserApiClient
/// can only be created from an [AccessToken].
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
    async fn get<T>(&self, query: &T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize,
    {
        let response = self
            .client
            .get(format!("{}/@me", USER_URL))
            .bearer_auth(&self.access_token)
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    /// Get information about the OAuth user
    ///
    /// Corresponds to the [Get my user information](https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_get) endpoint
    pub async fn get_my_user_information(
        &self,
        query: &GetUserInformation,
    ) -> Result<User, Box<dyn Error>> {
        let response = self.get(query).await?;
        let result: User = serde_json::from_str(response.as_str()).map_err(|err| {
            UserApiError::new(format!("Failed to parse AnimeList result: {}", err))
        })?;
        Ok(result)
    }
}

async fn handle_response(response: reqwest::Response) -> Result<String, Box<dyn Error>> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let content = response.text().await.map_err(|err| {
                UserApiError::new(format!("Failed to get content from response: {}", err))
            })?;
            Ok(content)
        }
        _ => Err(Box::new(UserApiError::new(format!(
            "Did not recieve OK response: {}",
            response.status()
        )))),
    }
}

use oauth2::AccessToken;
use reqwest;
use serde::Serialize;
use std::error::Error;

use crate::USER_URL;

use super::{error::UserApiError, requests::GetUserInformation, responses::User};

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

impl UserApiClient {
    async fn request<T>(&self, query: T) -> Result<String, Box<dyn Error>>
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

    pub async fn get_my_user_information(
        &self,
        query: GetUserInformation,
    ) -> Result<User, Box<dyn Error>> {
        let response = self.request(query).await?;
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
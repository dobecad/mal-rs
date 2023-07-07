use std::{error::Error, marker::PhantomData};

use async_trait::async_trait;
use oauth2::{AccessToken, ClientId};
use serde::de::DeserializeOwned;

use crate::{FORUM_URL, common::PagingIter};

use super::{
    error::ForumApiError,
    requests::{GetForumTopicDetail, GetForumTopics},
    responses::{ForumBoards, ForumTopicDetail, ForumTopics},
};

#[doc(hidden)]
#[derive(Debug)]
pub struct Client {}

#[doc(hidden)]
#[derive(Debug)]
pub struct Oauth {}

#[doc(hidden)]
#[derive(Debug)]
pub struct None {}

/// The ForumApiClient provides functions for interacting with the various
/// `forum` MAL API endpoints. The accessible endpoints do not vary between
/// [ClientId] or [AccessToken] clients.
/// 
/// # Examples
#[derive(Debug, Clone)]
pub struct ForumApiClient<State = None> {
    client: reqwest::Client,
    client_id: Option<String>,
    access_token: Option<String>,
    state: PhantomData<State>,
}

impl From<&AccessToken> for ForumApiClient<Oauth> {
    fn from(value: &AccessToken) -> Self {
        ForumApiClient::<Oauth> {
            client: reqwest::Client::new(),
            client_id: None,
            access_token: Some(value.secret().clone()),
            state: PhantomData::<Oauth>,
        }
    }
}

impl From<&ClientId> for ForumApiClient<Client> {
    fn from(value: &ClientId) -> Self {
        ForumApiClient::<Client> {
            client: reqwest::Client::new(),
            client_id: Some(value.clone().to_string()),
            access_token: None,
            state: PhantomData::<Client>,
        }
    }
}

/// This trait defines the common request methods available to both
/// Client and Oauth ForumApiClients
#[async_trait]
pub trait Request {
    async fn get(&self) -> Result<String, Box<dyn Error>>;

    async fn get_detail(&self, query: &GetForumTopicDetail) -> Result<String, Box<dyn Error>>;

    async fn get_topics(&self, query: &GetForumTopics) -> Result<String, Box<dyn Error>>;

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, Box<dyn Error>>;
}

/// This trait defines the shared endpoints for Client and Oauth
/// ForumApiClients. It provides default implementations such that
/// the Oauth ForumApiClient can override them if needed.
#[async_trait]
pub trait ForumApi {
    type State: Request + Send + Sync;

    /// Get a list of Forum boards
    /// 
    /// Corresponds to the [Get forum boards](https://myanimelist.net/apiconfig/references/api/v2#operation/forum_boards_get) endpoint
    async fn get_forum_boards(&self) -> Result<ForumBoards, Box<dyn Error>> {
        let response = self.get_self().get().await?;
        let result: ForumBoards = serde_json::from_str(response.as_str()).map_err(|err| {
            ForumApiError::new(format!("Failed to parse Forum Boards result: {}", err))
        })?;
        Ok(result)
    }

    /// Get details about a topic detail matching the given query
    /// 
    /// Corresponds to the [Get forum topic detail](https://myanimelist.net/apiconfig/references/api/v2#operation/forum_topic_get) endpoint
    async fn get_forum_topic_detail(
        &self,
        query: &GetForumTopicDetail,
    ) -> Result<ForumTopicDetail, Box<dyn Error>> {
        let response = self.get_self().get_detail(query).await?;
        let result: ForumTopicDetail = serde_json::from_str(response.as_str()).map_err(|err| {
            ForumApiError::new(format!(
                "Failed to parse Forum Topic Details result: {}",
                err
            ))
        })?;
        Ok(result)
    }

    /// Get a list of forum topics matching the given query
    /// 
    /// Corresponds to the [Get forum topics](https://myanimelist.net/apiconfig/references/api/v2#operation/forum_topics_get) endpoint
    async fn get_forum_topics(
        &self,
        query: &GetForumTopics,
    ) -> Result<ForumTopics, Box<dyn Error>> {
        let response = self.get_self().get_topics(query).await?;
        let result: ForumTopics = serde_json::from_str(response.as_str()).map_err(|err| {
            ForumApiError::new(format!("Failed to parse Forum Topics result: {}", err))
        })?;
        Ok(result)
    }

    /// Return the results of the next page, if possible
    async fn next<T, U>(&self, response: &U) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
        U: PagingIter + Sync + Send,
    {
        let response = self
            .get_self()
            .get_next_or_prev(response.next_page())
            .await?;
        let result: T = serde_json::from_str(response.as_str())
            .map_err(|err| ForumApiError::new(format!("Failed to fetch next page: {}", err)))?;
        Ok(result)
    }

    /// Return the results of the previous page, if possible
    async fn prev<T, U>(&self, response: &U) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
        U: PagingIter + Sync + Send,
    {
        let response = self
            .get_self()
            .get_next_or_prev(response.prev_page())
            .await?;
        let result: T = serde_json::from_str(response.as_str())
            .map_err(|err| ForumApiError::new(format!("Failed to fetch next page: {}", err)))?;
        Ok(result)
    }

    /// Utility method for API trait to use the appropriate request method
    fn get_self(&self) -> &Self::State;
}

#[async_trait]
impl Request for ForumApiClient<Client> {
    async fn get(&self) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/boards", FORUM_URL))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_detail(&self, query: &GetForumTopicDetail) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topic/{}", FORUM_URL, query.topic_id))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_topics(&self, query: &GetForumTopics) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topics", FORUM_URL))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, Box<dyn Error>> {
        if let Some(itr) = query {
            let response = self
                .client
                .get(itr)
                .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
                .send()
                .await?;

            handle_response(response).await
        } else {
            Err(Box::new(ForumApiError::new(
                "Page does not exist".to_string(),
            )))
        }
    }
}

#[async_trait]
impl Request for ForumApiClient<Oauth> {
    async fn get(&self) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/boards", FORUM_URL))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_detail(&self, query: &GetForumTopicDetail) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topic/{}", FORUM_URL, query.topic_id))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_topics(&self, query: &GetForumTopics) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topics", FORUM_URL))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, Box<dyn Error>> {
        if let Some(itr) = query {
            let response = self
                .client
                .get(itr)
                .bearer_auth(self.access_token.as_ref().unwrap())
                .send()
                .await?;

            handle_response(response).await
        } else {
            Err(Box::new(ForumApiError::new(
                "Page does not exist".to_string(),
            )))
        }
    }
}

impl ForumApi for ForumApiClient<Client> {
    type State = Self;

    fn get_self(&self) -> &Self::State {
        self
    }
}

impl ForumApi for ForumApiClient<Oauth> {
    type State = Self;

    fn get_self(&self) -> &Self::State {
        self
    }
}

async fn handle_response(response: reqwest::Response) -> Result<String, Box<dyn Error>> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let content = response.text().await.map_err(|err| {
                ForumApiError::new(format!("Failed to get content from response: {}", err))
            })?;
            Ok(content)
        }
        _ => Err(Box::new(ForumApiError::new(format!(
            "Did not recieve OK response: {}",
            response.status()
        )))),
    }
}

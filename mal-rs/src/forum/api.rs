use std::{error::Error, marker::PhantomData};

use async_trait::async_trait;
use oauth2::{AccessToken, ClientId};

use crate::FORUM_URL;

use super::{
    error::ForumApiError,
    requests::{GetForumTopicDetail, GetForumTopics},
    responses::{ForumBoards, ForumTopicDetail, ForumTopics},
};

#[derive(Debug)]
pub struct Client {}

#[derive(Debug)]
pub struct Oauth {}

#[derive(Debug)]
pub struct None {}

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

#[async_trait]
pub trait Request {
    async fn get(&self) -> Result<String, Box<dyn Error>>;

    async fn get_detail(&self, query: GetForumTopicDetail) -> Result<String, Box<dyn Error>>;

    async fn get_topics(&self, query: GetForumTopics) -> Result<String, Box<dyn Error>>;
}

#[async_trait]
pub trait ForumApi {
    type State: Request + Send + Sync;

    async fn get_forum_boards(&self) -> Result<ForumBoards, Box<dyn Error>> {
        let response = self.get_self().get().await?;
        let result: ForumBoards = serde_json::from_str(response.as_str()).map_err(|err| {
            ForumApiError::new(format!("Failed to parse Forum Boards result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_forum_topic_detail(
        &self,
        query: GetForumTopicDetail,
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

    async fn get_forum_topics(&self, query: GetForumTopics) -> Result<ForumTopics, Box<dyn Error>> {
        let response = self.get_self().get_topics(query).await?;
        let result: ForumTopics = serde_json::from_str(response.as_str()).map_err(|err| {
            ForumApiError::new(format!("Failed to parse Forum Topics result: {}", err))
        })?;
        Ok(result)
    }

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

    async fn get_detail(&self, query: GetForumTopicDetail) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topic/{}", FORUM_URL, query.topic_id))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_topics(&self, query: GetForumTopics) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topics", FORUM_URL))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
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

    async fn get_detail(&self, query: GetForumTopicDetail) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topic/{}", FORUM_URL, query.topic_id))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_topics(&self, query: GetForumTopics) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/topics", FORUM_URL))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
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
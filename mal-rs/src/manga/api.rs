// Wrapper for Manga API endpoint
use super::error::MangaApiError;
use super::requests::GetUserMangaList;
use async_trait::async_trait;
use oauth2::AccessToken;
use oauth2::ClientId;
use serde::Serialize;
use std::marker::PhantomData;

use crate::manga::requests::{DeleteMyMangaListItem, UpdateMyMangaListStatus};
use crate::{MANGA_URL, USER_URL};
use std::error::Error;

use super::{
    requests::{GetMangaDetails, GetMangaList, GetMangaRanking},
    responses::{MangaDetails, MangaList, MangaRanking},
};
use reqwest;

#[derive(Debug)]
pub struct Client {}

#[derive(Debug)]
pub struct Oauth {}

#[derive(Debug)]
pub struct None {}

#[derive(Debug, Clone)]
pub struct MangaApiClient<State = None> {
    client: reqwest::Client,
    client_id: Option<String>,
    access_token: Option<String>,
    state: PhantomData<State>,
}

impl From<&AccessToken> for MangaApiClient<Oauth> {
    fn from(value: &AccessToken) -> Self {
        MangaApiClient::<Oauth> {
            client: reqwest::Client::new(),
            client_id: None,
            access_token: Some(value.secret().clone()),
            state: PhantomData::<Oauth>,
        }
    }
}

impl From<&ClientId> for MangaApiClient<Client> {
    fn from(value: &ClientId) -> Self {
        MangaApiClient::<Client> {
            client: reqwest::Client::new(),
            client_id: Some(value.clone().to_string()),
            access_token: None,
            state: PhantomData::<Client>,
        }
    }
}

#[async_trait]
pub trait Request {
    async fn get<T>(&self, query: T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync;

    async fn get_details(&self, query: GetMangaDetails) -> Result<String, Box<dyn Error>>;

    async fn get_user(&self, query: GetUserMangaList) -> Result<String, Box<dyn Error>>;
}

#[async_trait]
impl Request for MangaApiClient<Client> {
    async fn get<T>(&self, query: T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        let response = self
            .client
            .get(MANGA_URL)
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_details(&self, query: GetMangaDetails) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}", MANGA_URL, query.manga_id))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_user(&self, query: GetUserMangaList) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}/mangalist", USER_URL, query.user_name))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }
}

#[async_trait]
impl Request for MangaApiClient<Oauth> {
    async fn get<T>(&self, query: T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        let response = self
            .client
            .get(MANGA_URL)
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_details(&self, query: GetMangaDetails) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}", MANGA_URL, query.manga_id))
            .bearer_auth(self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_user(&self, query: GetUserMangaList) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}/mangalist", USER_URL, query.user_name))
            .bearer_auth(self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }
}

#[async_trait]
pub trait MangaApi {
    type State: Request + Send + Sync;

    async fn get_manga_list(&self, query: GetMangaList) -> Result<MangaList, Box<dyn Error>> {
        let response = self.get_self().get(query).await?;
        let result: MangaList = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse MangaList result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_manga_details(
        &self,
        query: GetMangaDetails,
    ) -> Result<MangaDetails, Box<dyn Error>> {
        let response = self.get_self().get_details(query).await?;
        let result: MangaDetails = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse MangaList result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_manga_ranking(
        &self,
        query: GetMangaRanking,
    ) -> Result<MangaRanking, Box<dyn Error>> {
        let response = self.get_self().get(query).await?;
        let result: MangaRanking = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse MangaList result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_user_manga_list(
        &self,
        query: GetUserMangaList,
    ) -> Result<MangaList, Box<dyn Error>> {
        if query.user_name == "@me".to_string() {
            return Err(Box::new(MangaApiError::new(
                "You can only get your list via an Oauth client".to_string(),
            )));
        }
        let response = self.get_self().get_user(query).await?;
        let result: MangaList = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    fn get_self(&self) -> &Self::State;
}

#[async_trait]
impl MangaApi for MangaApiClient<Client> {
    type State = MangaApiClient<Client>;

    fn get_self(&self) -> &Self::State {
        self
    }
}

#[async_trait]
impl MangaApi for MangaApiClient<Oauth> {
    type State = MangaApiClient<Oauth>;

    fn get_self(&self) -> &Self::State {
        self
    }

    async fn get_user_manga_list(
        &self,
        query: GetUserMangaList,
    ) -> Result<MangaList, Box<dyn Error>> {
        let response = self.get_self().get_user(query).await?;
        let result: MangaList = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }
}

impl MangaApiClient<Oauth> {
    pub async fn update_manga_list_status(
        &self,
        query: UpdateMyMangaListStatus,
    ) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .patch(format!("{}/{}/my_list_status", MANGA_URL, query.manga_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn delete_manga_list_item(
        &self,
        query: DeleteMyMangaListItem,
    ) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .delete(format!("{}/{}/my_list_status", MANGA_URL, query.manga_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .send()
            .await?;

        handle_response(response).await
    }
}

async fn handle_response(response: reqwest::Response) -> Result<String, Box<dyn Error>> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let content = response.text().await.map_err(|err| {
                MangaApiError::new(format!("Failed to get content from response: {}", err))
            })?;
            Ok(content)
        }
        _ => Err(Box::new(MangaApiError::new(format!(
            "Did not recieve OK response: {}",
            response.status()
        )))),
    }
}

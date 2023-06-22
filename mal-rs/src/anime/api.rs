// Wrapper for Anime API endpoint
use super::error::AnimeApiError;
use async_trait::async_trait;
use oauth2::AccessToken;
use oauth2::ClientId;
use serde::Serialize;
use std::marker::PhantomData;

use crate::ANIME_URL;
use std::error::Error;

use super::{
    requests::{
        GetAnimeDetails, GetAnimeList, GetAnimeRanking, GetSeasonalAnime, GetSuggestedAnime,
    },
    responses::{AnimeDetails, AnimeList, AnimeRanking, SeasonalAnime, SuggestedAnime},
};
use reqwest;

#[derive(Debug)]
pub struct Client {}

#[derive(Debug)]
pub struct Oauth {}

#[derive(Debug)]
pub struct None {}

#[derive(Debug, Clone)]
pub struct AnimeApiClient<State = None> {
    client: reqwest::Client,
    client_id: Option<String>,
    access_token: Option<String>,
    state: PhantomData<State>,
}

impl From<&AccessToken> for AnimeApiClient<Oauth> {
    fn from(value: &AccessToken) -> Self {
        AnimeApiClient::<Oauth> {
            client: reqwest::Client::new(),
            client_id: None,
            access_token: Some(value.secret().clone()),
            state: PhantomData::<Oauth>,
        }
    }
}

impl From<&ClientId> for AnimeApiClient<Client> {
    fn from(value: &ClientId) -> Self {
        AnimeApiClient::<Client> {
            client: reqwest::Client::new(),
            client_id: Some(value.clone().to_string()),
            access_token: None,
            state: PhantomData::<Client>,
        }
    }
}

#[async_trait]
pub trait Request {
    async fn request<T>(&self, query: T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync;
}

#[async_trait]
pub trait AnimeApi {
    type State: Request + Send + Sync;

    async fn get_anime_list(&self, query: GetAnimeList) -> Result<AnimeList, Box<dyn Error>> {
        let response = self.get_self().request(query).await?;
        let result: AnimeList = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_anime_details(
        &self,
        query: GetAnimeDetails,
    ) -> Result<AnimeDetails, Box<dyn Error>> {
        let response = self.get_self().request(query).await?;
        let result: AnimeDetails = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_anime_ranking(
        &self,
        query: GetAnimeRanking,
    ) -> Result<AnimeRanking, Box<dyn Error>> {
        let response = self.get_self().request(query).await?;
        let result: AnimeRanking = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_seasonal_anime(
        &self,
        query: GetSeasonalAnime,
    ) -> Result<SeasonalAnime, Box<dyn Error>> {
        let response = self.get_self().request(query).await?;
        let result: SeasonalAnime = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
        })?;
        Ok(result)
    }

    fn get_self(&self) -> &Self::State;
}

#[async_trait]
impl Request for AnimeApiClient<Client> {
    async fn request<T>(&self, query: T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        let response = self
            .client
            .get(ANIME_URL)
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let content = response.text().await.map_err(|err| {
                    AnimeApiError::new(format!("Failed to get content from response: {}", err))
                })?;
                Ok(content)
            }
            _ => Err(Box::new(AnimeApiError::new(format!(
                "Did not recieve OK response: {}",
                response.status()
            )))),
        }
    }
}

#[async_trait]
impl Request for AnimeApiClient<Oauth> {
    async fn request<T>(&self, query: T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        let response = self
            .client
            .get(ANIME_URL)
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let content = response.text().await.map_err(|err| {
                    AnimeApiError::new(format!("Failed to get content from response: {}", err))
                })?;
                Ok(content)
            }
            _ => Err(Box::new(AnimeApiError::new(format!(
                "Did not recieve OK response: {}",
                response.status()
            )))),
        }
    }
}

#[async_trait]
impl AnimeApi for AnimeApiClient<Client> {
    type State = AnimeApiClient<Client>;

    fn get_self(&self) -> &Self::State {
        self
    }
}

#[async_trait]
impl AnimeApi for AnimeApiClient<Oauth> {
    type State = AnimeApiClient<Oauth>;

    fn get_self(&self) -> &Self::State {
        self
    }
}

impl AnimeApiClient<Oauth> {
    pub async fn get_suggested_anime(
        &self,
        query: GetSuggestedAnime,
    ) -> Result<SuggestedAnime, Box<dyn Error>> {
        let response = self.request(query).await?;
        let result: SuggestedAnime = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
        })?;
        Ok(result)
    }
}

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
pub struct Main {}

#[derive(Debug)]
pub struct None {}

enum AuthType {
    CLIENT,
    MAIN,
}

#[derive(Debug, Clone)]
pub struct AnimeApi<State = None> {
    client: reqwest::Client,
    client_id: Option<String>,
    access_token: Option<String>,
    state: PhantomData<State>,
}

impl From<AccessToken> for AnimeApi<Main> {
    fn from(value: AccessToken) -> Self {
        AnimeApi::<Main> {
            client: reqwest::Client::new(),
            client_id: None,
            access_token: Some(value.secret().clone()),
            state: PhantomData::<Main>,
        }
    }
}

impl From<ClientId> for AnimeApi<Client> {
    fn from(value: ClientId) -> Self {
        AnimeApi::<Client> {
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
pub trait Api {
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

// TODO: Maybe map errors to remove the error trait return in result
impl AnimeApi {
    // async fn get_request<T>(
    //     client: &reqwest::Client,
    //     query: T,
    //     token: &String,
    //     auth_type: AuthType,
    // ) -> Result<String, Box<dyn Error>>
    // where
    //     T: Serialize,
    // {
    //     let response = match auth_type {
    //         AuthType::CLIENT => {
    //             client
    //                 .get(ANIME_URL)
    //                 .header("X-MAL-CLIENT-ID", token)
    //                 .query(&query)
    //                 .send()
    //                 .await?
    //         }
    //         AuthType::MAIN => {
    //             client
    //                 .get(ANIME_URL)
    //                 .bearer_auth(token)
    //                 .query(&query)
    //                 .send()
    //                 .await?
    //         }
    //     };
    //     match response.status() {
    //         reqwest::StatusCode::OK => {
    //             let content = response.text().await.map_err(|err| {
    //                 AnimeApiError::new(format!("Failed to get content from response: {}", err))
    //             })?;
    //             Ok(content)
    //         }
    //         _ => Err(Box::new(AnimeApiError::new(format!(
    //             "Did not recieve OK response: {}",
    //             response.status()
    //         )))),
    //     }
    // }

    // fn select_token<'a, 'b>(
    //     client_id: &'a Option<String>,
    //     access_token: &'b Option<String>,
    // ) -> (&'a String, AuthType)
    // where
    //     'b: 'a,
    // {
    //     let (token, auth_type) = if let Some(client_id) = &client_id {
    //         (client_id, AuthType::CLIENT)
    //     } else {
    //         (access_token.as_ref().unwrap(), AuthType::MAIN)
    //     };
    //     (token, auth_type)
    // }

    // pub async fn get_anime_list(&self, query: GetAnimeList) -> Result<AnimeList, Box<dyn Error>> {
    //     let (token, auth_type) = AnimeApi::select_token(&self.client_id, &self.access_token);
    //     let response = AnimeApi::get_request(&self.client, query, token, auth_type).await?;
    //     let result: AnimeList = serde_json::from_str(response.as_str()).map_err(|err| {
    //         AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
    //     })?;
    //     Ok(result)
    // }

    // pub async fn get_anime_details(
    //     &self,
    //     query: GetAnimeDetails,
    // ) -> Result<AnimeDetails, Box<dyn Error>> {
    //     let (token, auth_type) = AnimeApi::select_token(&self.client_id, &self.access_token);
    //     let response = AnimeApi::get_request(&self.client, query, token, auth_type).await?;
    //     let result: AnimeDetails = serde_json::from_str(response.as_str()).map_err(|err| {
    //         AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
    //     })?;
    //     Ok(result)
    // }

    // pub async fn get_anime_ranking(
    //     &self,
    //     query: GetAnimeRanking,
    // ) -> Result<AnimeRanking, Box<dyn Error>> {
    //     let (token, auth_type) = AnimeApi::select_token(&self.client_id, &self.access_token);
    //     let response = AnimeApi::get_request(&self.client, query, token, auth_type).await?;
    //     let result: AnimeRanking = serde_json::from_str(response.as_str()).map_err(|err| {
    //         AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
    //     })?;
    //     Ok(result)
    // }

    // pub async fn get_seasonal_anime(
    //     &self,
    //     query: GetSeasonalAnime,
    // ) -> Result<SeasonalAnime, Box<dyn Error>> {
    //     let (token, auth_type) = AnimeApi::select_token(&self.client_id, &self.access_token);
    //     let response = AnimeApi::get_request(&self.client, query, token, auth_type).await?;
    //     let result: SeasonalAnime = serde_json::from_str(response.as_str()).map_err(|err| {
    //         AnimeApiError::new(format!("Failed to parse AnimeList result: {}", err))
    //     })?;
    //     Ok(result)
    // }
}

#[async_trait]
impl Request for AnimeApi<Client> {
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
impl Request for AnimeApi<Main> {
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
impl Api for AnimeApi<Client> {
    type State = AnimeApi<Client>;

    fn get_self(&self) -> &Self::State {
        self
    }
}

#[async_trait]
impl Api for AnimeApi<Main> {
    type State = AnimeApi<Main>;

    fn get_self(&self) -> &Self::State {
        self
    }
}

impl AnimeApi<Main> {
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

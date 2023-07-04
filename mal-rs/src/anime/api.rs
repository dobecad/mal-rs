// Wrapper for Anime API endpoint
use super::error::AnimeApiError;
use super::requests::DeleteMyAnimeListItem;
use super::requests::GetUserAnimeList;
use super::requests::UpdateMyAnimeListStatus;
use async_trait::async_trait;
use oauth2::AccessToken;
use oauth2::ClientId;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

use crate::common::PagingIter;
use crate::ANIME_URL;
use crate::USER_URL;
use std::error::Error;

use super::{
    requests::{
        GetAnimeDetails, GetAnimeList, GetAnimeRanking, GetSeasonalAnime, GetSuggestedAnime,
    },
    responses::{
        AnimeDetails, AnimeList, AnimeRanking, MyListStatus, SeasonalAnime, SuggestedAnime,
    },
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
    async fn get<T>(&self, query: &T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync;

    async fn get_details(&self, query: &GetAnimeDetails) -> Result<String, Box<dyn Error>>;

    async fn get_seasonal(&self, query: &GetSeasonalAnime) -> Result<String, Box<dyn Error>>;

    async fn get_user(&self, query: &GetUserAnimeList) -> Result<String, Box<dyn Error>>;

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, Box<dyn Error>>;
}

#[async_trait]
pub trait AnimeApi {
    type State: Request + Send + Sync;

    async fn get_anime_list(&self, query: &GetAnimeList) -> Result<AnimeList, Box<dyn Error>> {
        let response = self.get_self().get(query).await?;
        let result: AnimeList = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_anime_details(
        &self,
        query: &GetAnimeDetails,
    ) -> Result<AnimeDetails, Box<dyn Error>> {
        let response = self.get_self().get_details(query).await?;
        let result: AnimeDetails = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime Details result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_anime_ranking(
        &self,
        query: &GetAnimeRanking,
    ) -> Result<AnimeRanking, Box<dyn Error>> {
        let response = self.get_self().get(query).await?;
        let result: AnimeRanking = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime Ranking result: {}", err))
        })?;
        Ok(result)
    }

    async fn get_seasonal_anime(
        &self,
        query: &GetSeasonalAnime,
    ) -> Result<SeasonalAnime, Box<dyn Error>> {
        let response = self.get_self().get(query).await?;
        let result: SeasonalAnime = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Seasonal Anime result: {}", err))
        })?;
        Ok(result)
    }

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
            .map_err(|err| AnimeApiError::new(format!("Failed to fetch next page: {}", err)))?;
        Ok(result)
    }

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
            .map_err(|err| AnimeApiError::new(format!("Failed to fetch next page: {}", err)))?;
        Ok(result)
    }

    fn get_self(&self) -> &Self::State;
}

#[async_trait]
impl Request for AnimeApiClient<Client> {
    async fn get<T>(&self, query: &T) -> Result<String, Box<dyn Error>>
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

        handle_response(response).await
    }

    async fn get_details(&self, query: &GetAnimeDetails) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}", ANIME_URL, query.anime_id))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_seasonal(&self, query: &GetSeasonalAnime) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!(
                "{}/season/{}/{}",
                ANIME_URL, query.year, query.season
            ))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_user(&self, query: &GetUserAnimeList) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}/animelist", USER_URL, query.user_name))
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
            Err(Box::new(AnimeApiError::new(
                "Page does not exist".to_string(),
            )))
        }
    }
}

#[async_trait]
impl Request for AnimeApiClient<Oauth> {
    async fn get<T>(&self, query: &T) -> Result<String, Box<dyn Error>>
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

        handle_response(response).await
    }

    async fn get_details(&self, query: &GetAnimeDetails) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}", ANIME_URL, query.anime_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_seasonal(&self, query: &GetSeasonalAnime) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!(
                "{}/season/{}/{}",
                ANIME_URL, query.year, query.season
            ))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_user(&self, query: &GetUserAnimeList) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}/animelist", USER_URL, query.user_name))
            .bearer_auth(&self.access_token.as_ref().unwrap())
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
                .bearer_auth(&self.access_token.as_ref().unwrap())
                .send()
                .await?;

            handle_response(response).await
        } else {
            Err(Box::new(AnimeApiError::new(
                "Page does not exist".to_string(),
            )))
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

impl AnimeApiClient<Client> {
    pub async fn get_user_anime_list(
        &self,
        query: &GetUserAnimeList,
    ) -> Result<AnimeList, Box<dyn Error>> {
        if query.user_name == "@me".to_string() {
            return Err(Box::new(AnimeApiError::new(
                "You can only get your list via an Oauth client".to_string(),
            )));
        }
        let response = self.get_self().get_user(query).await?;
        let result: AnimeList = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
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
        query: &GetSuggestedAnime,
    ) -> Result<SuggestedAnime, Box<dyn Error>> {
        let response = self.get(query).await?;
        let result: SuggestedAnime = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Suggested Anime result: {}", err))
        })?;
        Ok(result)
    }

    pub async fn get_user_anime_list(
        &self,
        query: &GetUserAnimeList,
    ) -> Result<AnimeList, Box<dyn Error>> {
        let response = self.get_self().get_user(query).await?;
        let result: AnimeList = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    pub async fn update_anime_list_status(
        &self,
        query: UpdateMyAnimeListStatus,
    ) -> Result<MyListStatus, Box<dyn Error>> {
        let response = self
            .client
            .patch(format!("{}/{}/my_list_status", ANIME_URL, query.anime_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        let response = handle_response(response).await?;
        let result: MyListStatus = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    pub async fn delete_anime_list_item(
        &self,
        query: DeleteMyAnimeListItem,
    ) -> Result<(), Box<dyn Error>> {
        let response = self
            .client
            .delete(format!("{}/{}/my_list_status", ANIME_URL, query.anime_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => Err(Box::new(AnimeApiError::new(format!(
                "Did not recieve OK response: {}",
                response.status()
            )))),
        }
    }
}

async fn handle_response(response: reqwest::Response) -> Result<String, Box<dyn Error>> {
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

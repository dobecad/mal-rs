use super::{
    error::AnimeApiError,
    requests::{DeleteMyAnimeListItem, GetUserAnimeList, UpdateMyAnimeListStatus},
    responses::ListStatus,
};
use async_trait::async_trait;
use oauth2::{AccessToken, ClientId};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::{PhantomData, Send, Sync};

use crate::{
    common::{struct_to_form_data, PagingIter},
    oauth::{Authenticated, MalClientId, OauthClient},
    ANIME_URL, USER_URL,
};

use super::{
    requests::{
        GetAnimeDetails, GetAnimeList, GetAnimeRanking, GetSeasonalAnime, GetSuggestedAnime,
    },
    responses::{AnimeDetails, AnimeList, AnimeRanking, SeasonalAnime, SuggestedAnime},
};
use reqwest;

#[doc(hidden)]
#[derive(Debug)]
pub struct Client {}

#[doc(hidden)]
#[derive(Debug)]
pub struct Oauth {}

#[doc(hidden)]
#[derive(Debug)]
pub struct None {}

/// The AnimeApiClient provides functions for interacting with the various
/// `anime` and `user animelist` MAL API endpoints. The accessible endpoints
/// vary depending on if the AnimeApiClient was constructed from a
/// [MalClientId] or an [OauthClient].
///
/// Keep in mind that constructing an AnimeApiClient from a [OauthClient] provides
/// more access to the MAL API than from a [MalClientId]. Check the MAL API documentation
/// to view which endpoints require an [OauthClient] versus a [MalClientId] to see which
/// one is most appropriate for your use case.
///
/// # Example
///
/// ```rust,ignore
/// use dotenvy;
/// use mal_rs::oauth::MalClientId;
/// use mal_rs::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///     dotenvy::dotenv().ok();
///
///     let client_id = MalClientId::from_env().unwrap();
///     let api_client = AnimeApiClient::from(&client_id);
///     let common_fields = mal_rs::anime::all_common_fields();
///     let detail_fields = mal_rs::anime::all_detail_fields();
///
///     // Using the builder pattern for building the query
///     let query = GetAnimeList::builder("One Piece")
///         .fields(&common_fields)
///         .build()
///         .unwrap();
///     let response = api_client.get_anime_list(&query).await;
///     if let Ok(response) = response {
///         println!("Received response: {}\n", response);
///         for entry in response.data.iter() {
///             println!("Id: {}", entry.node.id);
///         }
///     }
///
///     let query = GetAnimeDetails::builder(9969)
///         .fields(&detail_fields)
///         .build()
///         .unwrap();
///     let response = api_client.get_anime_details(&query).await;
///     if let Ok(response) = response {
///         println!("Received response: {}\n", response);
///     }
/// }
/// ```
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

impl From<&MalClientId> for AnimeApiClient<Client> {
    fn from(value: &MalClientId) -> Self {
        AnimeApiClient::<Client> {
            client: reqwest::Client::new(),
            client_id: Some(value.0.to_string()),
            access_token: None,
            state: PhantomData::<Client>,
        }
    }
}

impl From<&OauthClient<Authenticated>> for AnimeApiClient<Oauth> {
    fn from(value: &OauthClient<Authenticated>) -> Self {
        AnimeApiClient {
            client: reqwest::Client::new(),
            client_id: None,
            access_token: Some(value.get_access_token().secret().clone()),
            state: PhantomData::<Oauth>,
        }
    }
}

/// This trait defines the common request methods available to both
/// Client and Oauth AnimeApiClients
#[async_trait]
pub trait Request {
    async fn get<T>(&self, query: &T) -> Result<String, AnimeApiError>
    where
        T: Serialize + Send + Sync;

    async fn get_details(&self, query: &GetAnimeDetails) -> Result<String, AnimeApiError>;

    async fn get_ranking(&self, query: &GetAnimeRanking) -> Result<String, AnimeApiError>;

    async fn get_seasonal(&self, query: &GetSeasonalAnime) -> Result<String, AnimeApiError>;

    async fn get_user(&self, query: &GetUserAnimeList) -> Result<String, AnimeApiError>;

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, AnimeApiError>;
}

/// This trait defines the shared endpoints for Client and Oauth
/// AnimeApiClients. It provides default implementations such that
/// the Oauth AnimeApiClient can override them if needed.
#[async_trait]
pub trait AnimeApi {
    type State: Request + Send + Sync;

    /// Get a list of anime that are similar to the given query
    ///
    /// Corresponds to the [Get anime list](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_get) endpoint
    async fn get_anime_list(&self, query: &GetAnimeList) -> Result<AnimeList, AnimeApiError> {
        let response = self
            .get_self()
            .get(query)
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed to get anime list: {}", err)))?;
        let result: AnimeList = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    /// Get the details of an anime that matches the given query
    ///
    /// Corresponds to the [Get anime details](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_get) endpoint
    async fn get_anime_details(
        &self,
        query: &GetAnimeDetails,
    ) -> Result<AnimeDetails, AnimeApiError> {
        let response =
            self.get_self().get_details(query).await.map_err(|err| {
                AnimeApiError::new(format!("Failed to get anime details: {}", err))
            })?;
        let result: AnimeDetails = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime Details result: {}", err))
        })?;
        Ok(result)
    }

    /// Get the ranking of anime
    ///
    /// Corresponds to the [Get anime ranking](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_ranking_get) endpoint
    async fn get_anime_ranking(
        &self,
        query: &GetAnimeRanking,
    ) -> Result<AnimeRanking, AnimeApiError> {
        let response =
            self.get_self().get_ranking(query).await.map_err(|err| {
                AnimeApiError::new(format!("Failed to get anime ranking: {}", err))
            })?;
        let result: AnimeRanking = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime Ranking result: {}", err))
        })?;
        Ok(result)
    }

    /// Get the seasonal anime that fall within the given query
    ///
    /// Corresponds to the [Get seasonal anime](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_season_year_season_get) endpoint
    async fn get_seasonal_anime(
        &self,
        query: &GetSeasonalAnime,
    ) -> Result<SeasonalAnime, AnimeApiError> {
        let response =
            self.get_self().get_seasonal(query).await.map_err(|err| {
                AnimeApiError::new(format!("Failed to get seasonal anime: {}", err))
            })?;
        let result: SeasonalAnime = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Seasonal Anime result: {}", err))
        })?;
        Ok(result)
    }

    /// Return the results of the next page, if possible
    async fn next<T>(&self, response: &T) -> Result<T, AnimeApiError>
    where
        T: DeserializeOwned + PagingIter + Sync + Send,
    {
        let response = self
            .get_self()
            .get_next_or_prev(response.next_page())
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed to fetch next page: {}", err)))?;
        let result: T = serde_json::from_str(response.as_str())
            .map_err(|err| AnimeApiError::new(format!("Failed to fetch next page: {}", err)))?;
        Ok(result)
    }

    /// Return the results of the previous page, if possible
    async fn prev<T>(&self, response: &T) -> Result<T, AnimeApiError>
    where
        T: DeserializeOwned + PagingIter + Sync + Send,
    {
        let response = self
            .get_self()
            .get_next_or_prev(response.prev_page())
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed to fetch previous page: {}", err)))?;
        let result: T = serde_json::from_str(response.as_str())
            .map_err(|err| AnimeApiError::new(format!("Failed to parse page: {}", err)))?;
        Ok(result)
    }

    /// Utility method for API trait to use the appropriate request method
    fn get_self(&self) -> &Self::State;
}

#[async_trait]
impl Request for AnimeApiClient<Client> {
    async fn get<T>(&self, query: &T) -> Result<String, AnimeApiError>
    where
        T: Serialize + Send + Sync,
    {
        let response = self
            .client
            .get(ANIME_URL)
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_details(&self, query: &GetAnimeDetails) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!("{}/{}", ANIME_URL, query.anime_id))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_ranking(&self, query: &GetAnimeRanking) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!("{}/ranking", ANIME_URL))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_seasonal(&self, query: &GetSeasonalAnime) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!(
                "{}/season/{}/{}",
                ANIME_URL, query.year, query.season
            ))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_user(&self, query: &GetUserAnimeList) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!("{}/{}/animelist", USER_URL, query.user_name))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, AnimeApiError> {
        if let Some(itr) = query {
            let response = self
                .client
                .get(itr)
                .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
                .send()
                .await
                .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

            handle_response(response).await
        } else {
            Err(AnimeApiError::new("Page does not exist".to_string()))
        }
    }
}

#[async_trait]
impl Request for AnimeApiClient<Oauth> {
    async fn get<T>(&self, query: &T) -> Result<String, AnimeApiError>
    where
        T: Serialize + Send + Sync,
    {
        let response = self
            .client
            .get(ANIME_URL)
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_details(&self, query: &GetAnimeDetails) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!("{}/{}", ANIME_URL, query.anime_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_ranking(&self, query: &GetAnimeRanking) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!("{}/ranking", ANIME_URL))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_seasonal(&self, query: &GetSeasonalAnime) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!(
                "{}/season/{}/{}",
                ANIME_URL, query.year, query.season
            ))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_user(&self, query: &GetUserAnimeList) -> Result<String, AnimeApiError> {
        let response = self
            .client
            .get(format!("{}/{}/animelist", USER_URL, query.user_name))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

        handle_response(response).await
    }

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, AnimeApiError> {
        if let Some(itr) = query {
            let response = self
                .client
                .get(itr)
                .bearer_auth(&self.access_token.as_ref().unwrap())
                .send()
                .await
                .map_err(|err| AnimeApiError::new(format!("Failed get request: {}", err)))?;

            handle_response(response).await
        } else {
            Err(AnimeApiError::new("Page does not exist".to_string()))
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
    /// Get a users anime list
    ///
    /// You **cannot** get the anime list of `@me` with a [ClientId] AnimeApiClient
    ///
    /// Corresponds to the [Get user anime list](https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_animelist_get) endpoint
    pub async fn get_user_anime_list(
        &self,
        query: &GetUserAnimeList,
    ) -> Result<AnimeList, AnimeApiError> {
        if query.user_name == "@me".to_string() {
            return Err(AnimeApiError::new(
                "You can only get your '@me' list via an Oauth client".to_string(),
            ));
        }
        let response = self.get_self().get_user(query).await.map_err(|err| {
            AnimeApiError::new(format!(
                "Failed to fetch {}'s anime list: {}",
                query.user_name, err
            ))
        })?;
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
    /// Get a list of suggested anime
    ///
    /// Corresponds to the [Get suggested anime](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_suggestions_get) endpoint
    pub async fn get_suggested_anime(
        &self,
        query: &GetSuggestedAnime,
    ) -> Result<SuggestedAnime, AnimeApiError> {
        let response = self
            .client
            .get(format!("{}/suggestions", ANIME_URL))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await
            .map_err(|err| {
                AnimeApiError::new(format!("Failed to fetch suggested anime: {}", err))
            })?;

        let response = handle_response(response).await?;

        let result: SuggestedAnime = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Suggested Anime result: {}", err))
        })?;
        Ok(result)
    }

    /// Get a users Anime list
    ///
    /// You **can** get the anime list of `@me` with an [Oauth] AnimeApiClient
    ///
    /// Corresponds to the [Get user anime list](https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_animelist_get) endpoint
    pub async fn get_user_anime_list(
        &self,
        query: &GetUserAnimeList,
    ) -> Result<AnimeList, AnimeApiError> {
        let response =
            self.get_self().get_user(query).await.map_err(|err| {
                AnimeApiError::new(format!("Failed to get user anime list: {}", err))
            })?;
        let result: AnimeList = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    /// Update the status of an anime for the OAuth user's anime list
    ///
    /// Corresponds to the [Update my anime list status](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_put) endpoint
    pub async fn update_anime_list_status(
        &self,
        query: &UpdateMyAnimeListStatus,
    ) -> Result<ListStatus, AnimeApiError> {
        let form_data = struct_to_form_data(&query).map_err(|err| {
            AnimeApiError::new(format!("Failed to turn request into form data: {}", err))
        })?;
        let response = self
            .client
            .put(format!("{}/{}/my_list_status", ANIME_URL, query.anime_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .form(&form_data)
            .send()
            .await
            .map_err(|err| {
                AnimeApiError::new(format!(
                    "Failed to update user's anime list status: {}",
                    err
                ))
            })?;

        let response = handle_response(response).await?;
        let result: ListStatus = serde_json::from_str(response.as_str()).map_err(|err| {
            AnimeApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    /// Delete an anime entry from the OAuth user's anime list
    ///
    /// Corresponds to the [Delete my anime list item](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_delete) endpoint
    pub async fn delete_anime_list_item(
        &self,
        query: &DeleteMyAnimeListItem,
    ) -> Result<(), AnimeApiError> {
        let response = self
            .client
            .delete(format!("{}/{}/my_list_status", ANIME_URL, query.anime_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .send()
            .await
            .map_err(|err| {
                AnimeApiError::new(format!("Failed to delete the anime list item: {}", err))
            })?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::NOT_FOUND => Err(AnimeApiError::new(
                "Anime does not exist in user's anime list".to_string(),
            )),
            _ => Err(AnimeApiError::new(format!(
                "Did not recieve expected response: {}",
                response.status()
            ))),
        }
    }
}

async fn handle_response(response: reqwest::Response) -> Result<String, AnimeApiError> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let content = response.text().await.map_err(|err| {
                AnimeApiError::new(format!("Failed to get content from response: {}", err))
            })?;
            Ok(content)
        }
        _ => Err(AnimeApiError::new(format!(
            "Did not recieve OK response: {}",
            response.status()
        ))),
    }
}

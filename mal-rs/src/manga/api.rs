use super::{error::MangaApiError, requests::GetUserMangaList, responses::ListStatus};
use async_trait::async_trait;
use oauth2::{AccessToken, ClientId};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

use crate::{
    common::{struct_to_form_data, PagingIter},
    manga::requests::{DeleteMyMangaListItem, UpdateMyMangaListStatus},
    oauth::{Authenticated, MalClientId, OauthClient},
    MANGA_URL, USER_URL,
};
use std::error::Error;

use super::{
    requests::{GetMangaDetails, GetMangaList, GetMangaRanking},
    responses::{MangaDetails, MangaList, MangaRanking},
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

/// The MangaApiClient provides functions for interacting with the various
/// `manga` and `user mangalist` MAL API endpoints. The accessible endpoints
/// vary depending on if the MangaApiClient was constructed from a
/// [ClientId] or an [AccessToken].
///
/// Keep in mind that constructing an MangaApiClient from a [AccessToken] provides
/// more access to the MAL API than from a [ClientId]. Check the MAL API documentation
/// to view which endpoints require an [AccessToken] versus a [ClientId] to see which
/// one is most appropriate for your use case.
///
/// # Examples
///
/// ## Using ClientId
/// ```rust,ignore
/// use std::env;
///
/// use dotenv;
/// use mal_rs::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///     dotenv::dotenv().ok();
///
///     let client_id = ClientId::new(
///         env::var("CLIENT_ID").expect("CLIENT_ID environment variable is not defined"),
///     );
///
///     // Create MangaApiClient from the ClientId
///     let api_client = MangaApiClient::from(&client_id);
/// }
/// ```
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

impl From<&MalClientId> for MangaApiClient<Client> {
    fn from(value: &MalClientId) -> Self {
        MangaApiClient::<Client> {
            client: reqwest::Client::new(),
            client_id: Some(value.0.to_string()),
            access_token: None,
            state: PhantomData::<Client>,
        }
    }
}

impl From<&OauthClient<Authenticated>> for MangaApiClient<Oauth> {
    fn from(value: &OauthClient<Authenticated>) -> Self {
        MangaApiClient {
            client: reqwest::Client::new(),
            client_id: None,
            access_token: Some(value.get_access_token().secret().clone()),
            state: PhantomData::<Oauth>,
        }
    }
}

/// This trait defines the common request methods available to both
/// Client and Oauth MangaApiClients
#[async_trait]
pub trait Request {
    async fn get<T>(&self, query: &T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync;

    async fn get_details(&self, query: &GetMangaDetails) -> Result<String, Box<dyn Error>>;

    async fn get_ranking(&self, query: &GetMangaRanking) -> Result<String, Box<dyn Error>>;

    async fn get_user(&self, query: &GetUserMangaList) -> Result<String, Box<dyn Error>>;

    async fn get_next_or_prev(&self, query: Option<&String>) -> Result<String, Box<dyn Error>>;
}

#[async_trait]
impl Request for MangaApiClient<Client> {
    async fn get<T>(&self, query: &T) -> Result<String, Box<dyn Error>>
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

    async fn get_details(&self, query: &GetMangaDetails) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}", MANGA_URL, query.manga_id))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_ranking(&self, query: &GetMangaRanking) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/ranking", MANGA_URL))
            .header("X-MAL-CLIENT-ID", self.client_id.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_user(&self, query: &GetUserMangaList) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}/mangalist", USER_URL, query.user_name))
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
            Err(Box::new(MangaApiError::new(
                "Page does not exist".to_string(),
            )))
        }
    }
}

#[async_trait]
impl Request for MangaApiClient<Oauth> {
    async fn get<T>(&self, query: &T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        let response = self
            .client
            .get(MANGA_URL)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_details(&self, query: &GetMangaDetails) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}", MANGA_URL, query.manga_id))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_ranking(&self, query: &GetMangaRanking) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/ranking", MANGA_URL))
            .bearer_auth(self.access_token.as_ref().unwrap())
            .query(&query)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_user(&self, query: &GetUserMangaList) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/{}/mangalist", USER_URL, query.user_name))
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
            Err(Box::new(MangaApiError::new(
                "Page does not exist".to_string(),
            )))
        }
    }
}

/// This trait defines the shared endpoints for Client and Oauth
/// MangaApiClients. It provides default implementations such that
/// the Oauth MangaApiClient can override them if needed.
#[async_trait]
pub trait MangaApi {
    type State: Request + Send + Sync;

    /// Get a list of manga that are similar to the given query
    ///
    /// Corresponds to the [Get manga list](https://myanimelist.net/apiconfig/references/api/v2#operation/manga_get) endpoint
    async fn get_manga_list(&self, query: &GetMangaList) -> Result<MangaList, Box<dyn Error>> {
        let response = self.get_self().get(query).await?;
        let result: MangaList = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse MangaList result: {}", err))
        })?;
        Ok(result)
    }

    /// Get the details of a manga that matches the given query
    ///
    /// Corresponds to the [Get manga details](https://myanimelist.net/apiconfig/references/api/v2#operation/manga_manga_id_get) endpoint
    async fn get_manga_details(
        &self,
        query: &GetMangaDetails,
    ) -> Result<MangaDetails, Box<dyn Error>> {
        let response = self.get_self().get_details(query).await?;
        let result: MangaDetails = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse MangaList result: {}", err))
        })?;
        Ok(result)
    }

    /// Get the ranking of manga
    ///
    /// Corresponds to the [Get manga ranking](https://myanimelist.net/apiconfig/references/api/v2#operation/manga_ranking_get) endpoint
    async fn get_manga_ranking(
        &self,
        query: &GetMangaRanking,
    ) -> Result<MangaRanking, Box<dyn Error>> {
        let response = self.get_self().get_ranking(query).await?;
        let result: MangaRanking = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse MangaList result: {}", err))
        })?;
        Ok(result)
    }

    /// Get a users manga list
    ///
    /// You **cannot** get the manga list of `@me` with a [ClientId] MangaApiClient
    ///
    /// Corresponds to the [Get user mangalist](https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_mangalist_get) endpoint
    async fn get_user_manga_list(
        &self,
        query: &GetUserMangaList,
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
            .map_err(|err| MangaApiError::new(format!("Failed to fetch next page: {}", err)))?;
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
            .map_err(|err| MangaApiError::new(format!("Failed to fetch next page: {}", err)))?;
        Ok(result)
    }

    /// Utility method for API trait to use the appropriate request method
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

    /// Get a users manga list
    ///
    /// You **can** get the manga list of `@me` with an [Oauth] MangaApiClient
    ///
    /// Corresponds to the [Get user mangalist](https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_mangalist_get) endpoint
    async fn get_user_manga_list(
        &self,
        query: &GetUserMangaList,
    ) -> Result<MangaList, Box<dyn Error>> {
        let response = self.get_self().get_user(query).await?;
        let result: MangaList = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }
}

impl MangaApiClient<Oauth> {
    /// Update the status of a manga for the OAuth user's manga list
    ///
    /// Correspoonds to the [Update my manga list status](https://myanimelist.net/apiconfig/references/api/v2#operation/manga_manga_id_my_list_status_put) endpoint
    pub async fn update_manga_list_status(
        &self,
        query: &UpdateMyMangaListStatus,
    ) -> Result<ListStatus, Box<dyn Error>> {
        let form_data = struct_to_form_data(&query)?;
        let response = self
            .client
            .put(format!("{}/{}/my_list_status", MANGA_URL, query.manga_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .form(&form_data)
            .send()
            .await?;

        let response = handle_response(response).await?;
        let result: ListStatus = serde_json::from_str(response.as_str()).map_err(|err| {
            MangaApiError::new(format!("Failed to parse Anime List result: {}", err))
        })?;
        Ok(result)
    }

    /// Delete a manga entry from the OAuth user's manga list
    ///
    /// Corresponds to the [Delete my manga list item](https://myanimelist.net/apiconfig/references/api/v2#operation/manga_manga_id_my_list_status_delete) endpoint
    pub async fn delete_manga_list_item(
        &self,
        query: &DeleteMyMangaListItem,
    ) -> Result<(), Box<dyn Error>> {
        let response = self
            .client
            .delete(format!("{}/{}/my_list_status", MANGA_URL, query.manga_id))
            .bearer_auth(&self.access_token.as_ref().unwrap())
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::NOT_FOUND => Err(Box::new(MangaApiError::new(
                "Manga does not exist in user's manga list".to_string(),
            ))),
            _ => Err(Box::new(MangaApiError::new(format!(
                "Did not recieve expected response: {}",
                response.status()
            )))),
        }
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

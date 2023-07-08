//! mal-rs is an asynchronous, fully type-safe MyAnimeList API
//!
//! # Table of Contents
//! - [Overview](#overview)
//! - [OAuth](#oauth)
//! - [API Clients](#api-clients)
//! - [Anime and Manga Fields](#anime-and-manga-fields)
//! - [Examples](#examples)
//!
//! # Overview
//!
//! mal-rs library is a fully type-safe library
//! that provides asynchronous functionality for interacting with the [MyAnimeList](https://myanimelist.net/apiconfig/references/api/v2) (MAL)
//! API. Built with Rust's async/await syntax and strong type system, this
//! library ensures efficient and safe handling of API requests and responses
//! while leveraging the benefits of asynchronous programming.
//!
//! With mal-rs, developers can confidently integrate MAL API
//! functionality into their applications, enabling them to retrieve, update,
//! and manage anime and manga data effortlessly. The library offers a comprehensive
//! set of API endpoints, allowing users to perform operations such as searching for
//! anime, retrieving detailed information about specific titles, managing user
//! lists, and much more.
//!
//! One of the key features of mal-rs is its type safety. By utilizing Rust's
//! strong type system, the library provides compile-time guarantees that the API
//! requests and responses are correctly structured and formatted. This eliminates
//! the risk of runtime errors and enhances code reliability. Developers can
//! leverage the library's well-defined structs and enums to easily construct API
//! requests and handle the received data in a type-safe manner.
//!
//! # OAuth
//!
//! mal-rs provides a method for obtaining a MAL OAuth token.
//! This token is necessary to access certain MAL API endpoints.
//! Depending on whether you obtain an OAuth token or just use your ClientId,
//! the mal-rs API client you create from either token will ensure you can only
//! access the endpoints your token is comptatible with.
//!
//! # API Clients
//!
//! There are four different API clients you can use:
//! - APIClient
//!   - Implements all of the [anime](https://myanimelist.net/apiconfig/references/api/v2#tag/anime)
//! and [user animelist](https://myanimelist.net/apiconfig/references/api/v2#tag/user-animelist) MAL API endpoints
//!   - Can be created from an AccessToken or a ClientId
//! - MangaClient
//!     - Implements all of the [manga](https://myanimelist.net/apiconfig/references/api/v2#tag/manga)
//! and [user mangalist](https://myanimelist.net/apiconfig/references/api/v2#tag/user-mangalist) MAL API endpoints
//!     - Can be created from an AccessToken or a ClientId
//! - ForumClient
//!     - Implements all of the [forum](https://myanimelist.net/apiconfig/references/api/v2#tag/forum) MAL API endpoints
//!     - Can be created from an AccessToken or a ClientId
//! - UserClient
//!     - Implements all of the [user](https://myanimelist.net/apiconfig/references/api/v2#tag/user) MAL API endpoints
//!     - Can be created from an AccessToken
//!
//! # Anime and Manga Fields
//!
//! mal-rs provides utilities to ensure that the fields you want returned from the
//! anime and manga endpoints are valid fields.
//!
//! ```rust,no_run
//! use mal_rs::prelude::*;
//! use mal_rs::anime_fields;
//!
//!     // Anime Fields example
//!     let fields = anime_fields!(
//!         AnimeFieldsEnum::id,
//!         AnimeFieldsEnum::num_episodes,
//!         AnimeFieldsEnum::title,
//!         // ....
//!     );
//!
//!     // If you want all the fields:
//!     let fields = mal_rs::anime::all_fields();
//! ```
//!
//! # Examples
//!
//! ## Using a ClientId
//!
//! ```rust,no_run
//! use std::env;
//!
//! use dotenv;
//! use mal_rs::prelude::*;
//! use mal_rs::{anime_fields, manga_fields};
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv::dotenv().ok();
//!
//!     let client_id = ClientId::new(
//!         env::var("CLIENT_ID").expect("CLIENT_ID environment variable is not defined"),
//!     );
//!
//!     // Create AnimeApiClient from the client_id
//!     let api_client = AnimeApiClient::from(&client_id);
//!
//!     // Specify the anime fields you want returned
//!     let fields = anime_fields!(
//!         AnimeFieldsEnum::id,
//!         AnimeFieldsEnum::num_episodes,
//!         AnimeFieldsEnum::title,
//!     );
//!
//!     // Create a type-safe query to pass to the API client
//!     // Search for an anime named "one", limit to 5 results, no offset, with the given fields
//!     let query = GetAnimeList::new("one".to_string(), Some(5), None, Some(&fields)).unwrap();
//!     let result = api_client.get_anime_list(&query).await.unwrap();
//!     println!("Result: {}", &result);
//!
//!     // Example iterating through pages
//!     let result: AnimeList = api_client.next(&result).await.unwrap();
//!     println!("Next result: {}", &result);
//!
//!     let result: AnimeList = api_client.prev(&result).await.unwrap();
//!     println!("Prev result: {}", &result);
//!
//!     // Manga API example
//!     let api_client = MangaApiClient::from(&client_id);
//!     let fields = mal_rs::manga::all_fields();
//!     let query = GetMangaList::new("one".to_string(), Some(5), None, Some(&fields)).unwrap();
//!     let result = api_client.get_manga_list(&query).await.unwrap();
//!     println!("Result: {}", result);
//! }
//! ```
//!
//! ## Using OAuth token
//!
//! ```rust,no_run
//! use dotenv;
//! use mal_rs::{
//!     oauth::{OauthClient, RedirectResponse},
//!     user::{
//!         api::UserApiClient,
//!         requests::{GetUserInformation, UserFields},
//!         responses::UserEnum,
//!     }, user_fields,
//! };
//! use std::io;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv::dotenv().ok();
//!
//!     let mut oauth_client = OauthClient::new();
//!     println!(
//!         "Visit this URL: {}\n",
//!         oauth_client.generate_readonly_auth_url()
//!     );
//!
//!     println!("After authorizing, please enter the URL you were redirected to: ");
//!     let mut input = String::new();
//!     io::stdin()
//!         .read_line(&mut input)
//!         .expect("Failed to read user input");
//!
//!     let response = RedirectResponse::try_from(input).unwrap();
//!
//!     // Authenticate to get an Authenticated oauth_client back
//!     let result = oauth_client.authenticate(response).await;
//!     let authenticated_oauth_client = match result {
//!         Ok(t) => {
//!             println!("Got token: {:?}\n", t.get_access_token_secret());
//!     
//!             let t = t.refresh().await.unwrap();
//!             println!("Refreshed token: {:?}", t.get_access_token_secret());
//!             t
//!         }
//!         Err(e) => panic!("Failed: {}", e),
//!     };
//!     
//!     // Create UserApiClient from the OauthClient
//!     let api_client = UserApiClient::from(&authenticated_oauth_client);
//! 
//!     // Create fields that you want returned by the MAL API
//!     let fields = user_fields!(UserEnum::id, UserEnum::name, UserEnum::is_supporter);
//!     let query = GetUserInformation::new(Some(&fields));
//!     let response = api_client.get_my_user_information(&query).await.unwrap();
//!     println!("Information about yourself: {:?}", response);
//! }
//! ```

#[cfg(feature = "anime")]
pub mod anime;

#[cfg(feature = "manga")]
pub mod manga;

#[cfg(feature = "forum")]
pub mod forum;

#[cfg(feature = "user")]
pub mod user;

pub mod common;
pub mod macros;
pub mod oauth;

const OAUTH_URL: &'static str = "https://myanimelist.net/v1/oauth2/authorize";
const OAUTH_TOKEN_URL: &'static str = "https://myanimelist.net/v1/oauth2/token";

#[cfg(feature = "anime")]
const ANIME_URL: &'static str = "https://api.myanimelist.net/v2/anime";

#[cfg(feature = "manga")]
const MANGA_URL: &'static str = "https://api.myanimelist.net/v2/manga";

#[cfg(feature = "forum")]
const FORUM_URL: &'static str = "https://api.myanimelist.net/v2/forum";

#[cfg(any(feature = "anime", feature = "manga", feature = "user"))]
const USER_URL: &'static str = "https://api.myanimelist.net/v2/users";

/// Module re-exports
pub mod prelude {
    #[cfg(feature = "anime")]
    pub use crate::anime::{api::*, requests::*, responses::*};

    #[cfg(feature = "manga")]
    pub use crate::manga::{api::*, requests::*, responses::*};

    #[cfg(feature = "forum")]
    pub use crate::forum::{api::*, requests::*, responses::*};

    #[cfg(feature = "user")]
    pub use crate::user::{api::*, requests::*, responses::*};

    pub use crate::oauth::{MalClientId, OauthClient};
}

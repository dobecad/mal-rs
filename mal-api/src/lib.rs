//! `mal-api` is an asynchronous, fully type-safe MyAnimeList API
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
//! `mal-api` library is a fully type-safe library
//! that provides asynchronous functionality for interacting with the [MyAnimeList](https://myanimelist.net/apiconfig/references/api/v2) (MAL)
//! API.
//!
//! With `mal-api`, developers can confidently integrate MAL API
//! functionality into their applications, enabling them to retrieve, update,
//! and manage anime and manga data effortlessly. The library offers a comprehensive
//! set of API endpoints, allowing users to perform operations such as searching for
//! anime, retrieving detailed information about specific titles, managing user
//! lists, and more.
//!
//! One of the key features of `mal-api` is its type safety. By utilizing Rust's
//! strong type system, the library provides compile-time guarantees that the API
//! requests and responses are correctly structured and formatted. This eliminates
//! the risk of runtime errors. Developers can leverage the library's well-defined
//! structs and enums to easily construct API requests and handle the received
//! data in a type-safe manner.
//!
//! # OAuth
//!
//! `mal-api` provides a method for obtaining MAL OAuth access tokens.
//! This token is necessary to access certain MAL API endpoints.
//! Depending on whether you obtain an OAuth token or just use your ClientId,
//! the `mal-api` API client you create from either token will ensure you can only
//! access the endpoints your token is comptatible with.
//!
//! # API Clients
//!
//! There are four different API clients you can use:
//! - AnimeApiClient
//!   - Implements all of the [anime](https://myanimelist.net/apiconfig/references/api/v2#tag/anime)
//! and [user animelist](https://myanimelist.net/apiconfig/references/api/v2#tag/user-animelist) MAL API endpoints
//!   - Can be created from a MAL Oauth access token or a MAL ClientId
//! - MangaApiClient
//!     - Implements all of the [manga](https://myanimelist.net/apiconfig/references/api/v2#tag/manga)
//! and [user mangalist](https://myanimelist.net/apiconfig/references/api/v2#tag/user-mangalist) MAL API endpoints
//!     - Can be created from a MAL Oauth access token or a MAL ClientId
//! - ForumApiClient
//!     - Implements all of the [forum](https://myanimelist.net/apiconfig/references/api/v2#tag/forum) MAL API endpoints
//!     - Can be created from a MAL Oauth access token or a MAL ClientId
//! - UserApiClient
//!     - Implements all of the [user](https://myanimelist.net/apiconfig/references/api/v2#tag/user) MAL API endpoints
//!     - Can be created from a MAL Oauth access token
//!
//! # Anime and Manga Fields
//!
//! `mal-api` provides utilities to ensure that the fields you want returned from the
//! anime and manga endpoints are valid fields.
//!
//! ```rust,no_run
//! use mal_api::prelude::*;
//! use mal_api::anime_common_fields;
//!
//! // Specify which fields you want returned from the Anime endpoint
//! let fields = anime_common_fields!(
//!     AnimeField::id,
//!     AnimeField::num_episodes,
//!     AnimeField::title,
//! );
//!
//! // If you want all of the common fields:
//! let fields = mal_api::anime::all_common_fields();
//!
//! // If you want all of the detailed fields:
//! let fields = mal_api::anime::all_detail_fields();
//! ```
//!
//! # Examples
//!
//! ## Using a ClientId
//!
//! ```rust,ignore
//! use dotenvy;
//! use mal_api::anime_common_fields;
//! use mal_api::oauth::MalClientId;
//! use mal_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenvy::dotenv().ok();
//!
//!     let client_id = MalClientId::try_from_env().unwrap();
//!
//!     // Anime API example
//!     let api_client = AnimeApiClient::from(&client_id);
//!     let fields = anime_common_fields!(
//!         AnimeField::id,
//!         AnimeField::num_episodes,
//!         AnimeField::title,
//!     );
//!
//!     // Example using builder pattern. The `builder(args...)` method will only require
//!     // the required arguments for the specific API endpoint, while the
//!     // other builder instance methods will build up the optional arguments.
//!     let query = GetAnimeList::builder("One")
//!         .fields(&fields)
//!         .limit(5)
//!         .build()
//!         .unwrap();
//!     let result = api_client.get_anime_list(&query).await.unwrap();
//!     println!("Result: {}", &result);
//!
//!     // Example iterating through pages
//!     let result = api_client.next(&result).await.unwrap();
//!     println!("Next result: {}", &result);
//!
//!     let result = api_client.prev(&result).await.unwrap();
//!     println!("Prev result: {}", &result);
//!
//!     // Manga API example
//!     let api_client = MangaApiClient::from(&client_id);
//!     let fields = mal_api::manga::all_common_fields();
//!
//!     // Example using `new` pattern. Not recommended, but available
//!     let nsfw = false;
//!     let limit = Some(5);
//!     let query = GetMangaList::new("one".to_string(), nsfw, Some(&fields), limit, None).unwrap();
//!     let result = api_client.get_manga_list(&query).await.unwrap();
//!     println!("Result: {}", result);
//! }
//! ```
//!
//! ## Creating an OAuth token
//!
//! ```rust,ignore
//! use dotenvy;
//! use mal_api::oauth::{OauthClient, RedirectResponse};
//! use std::io;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenvy::dotenv().ok();
//!
//!     let authenticated_client = OauthClient::load_from_config();
//!     match authenticated_client {
//!         Ok(_) => {
//!             println!("An existing authorized Oauth client already exists");
//!             return;
//!         }
//!         Err(_) => println!("No existing Oauth client exists\n"),
//!     }
//!
//!     let mut oauth_client = OauthClient::new().unwrap();
//!     println!("Visit this URL: {}\n", oauth_client.generate_auth_url());
//!
//!     println!("After authorizing, please enter the URL you were redirected to: ");
//!     let mut input = String::new();
//!     io::stdin()
//!         .read_line(&mut input)
//!         .expect("Failed to read user input");
//!
//!     let response = RedirectResponse::try_from(input).unwrap();
//!
//!     // Authentication process
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
//!     // Save credentials to config to be re-used later
//!     let _ = authenticated_oauth_client.save_to_config();
//! }
//! ```
//!
//! ## Accessing data from responses
//! ```rust,ignore
//! let query = GetAnimeList::builder("One Piece")
//!     .fields(&common_fields)
//!     .build()
//!     .unwrap();
//! let response = api_client.get_anime_list(&query).await;
//! if let Ok(response) = response {
//!     // Iterate through all of the anime entries, printing each anime's title and id
//!     for entry in response.data.iter() {
//!         println!("Anime Title: {}  Anime ID: {}", entry.node.title, entry.node.id);
//!     }
//! }
//! ```

pub mod anime;
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
const ANIME_URL: &'static str = "https://api.myanimelist.net/v2/anime";
const MANGA_URL: &'static str = "https://api.myanimelist.net/v2/manga";
const USER_URL: &'static str = "https://api.myanimelist.net/v2/users";

#[cfg(feature = "forum")]
const FORUM_URL: &'static str = "https://api.myanimelist.net/v2/forum";

/// Module re-exports
pub mod prelude {
    pub use crate::oauth::{MalClientId, OauthClient};

    pub use crate::anime::{
        api::{AnimeApi, AnimeApiClient},
        requests::*,
        responses::*,
    };

    pub use crate::manga::{
        api::{MangaApi, MangaApiClient},
        requests::*,
        responses::*,
    };

    #[cfg(feature = "forum")]
    pub use crate::forum::{
        api::{ForumApi, ForumApiClient},
        requests::*,
        responses::*,
    };

    #[cfg(feature = "user")]
    pub use crate::user::{api::UserApiClient, requests::*};
}

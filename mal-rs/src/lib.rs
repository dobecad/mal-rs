pub mod anime;
pub mod manga;
pub mod oauth;
pub mod user;
pub mod common;

use serde::{Deserialize};

const OAUTH_URL: &'static str = "https://myanimelist.net/v1/oauth2/authorize";
const OAUTH_TOKEN_URL: &'static str = "https://myanimelist.net/v1/oauth2/token";
const ANIME_URL: &'static str = "https://api.myanimelist.net/v2/anime";
const MANGA_URL: &'static str = "https://api.myanimelist.net/v2/manga";
const USER_URL: &'static str = "https://api.myanimelist.net/v2/users";

#[cfg(test)]
mod tests {
    use super::*;
}

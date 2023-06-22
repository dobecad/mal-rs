pub mod anime;
pub mod manga;
pub mod oauth;
pub mod user;

use serde::{Deserialize, Serialize};

const OAUTH_URL: &'static str = "https://myanimelist.net/v1/oauth2/authorize";
const OAUTH_TOKEN_URL: &'static str = "https://myanimelist.net/v1/oauth2/token";
const ANIME_URL: &'static str = "https://api.myanimelist.net/v2/anime";
const MANGA_URL: &'static str = "https://api.myanimelist.net/v2/manga";
const USER_URL: &'static str = "https://api.myanimelist.net/v2/user";


#[derive(Debug, Deserialize)]
pub struct Paging {
    previous: String,
    next: String,
}

#[derive(Debug, Deserialize)]
pub struct MainPicture {
    medium: String,
    large: String,
}

#[derive(Debug, Deserialize)]
pub struct AlternativeTitles {
    synonyms: Option<Vec<String>>,
    en: Option<String>,
    ja: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum NSFW {
    #[serde(rename = "white")]
    SFW,
    #[serde(rename = "gray")]
    MNSFW,
    #[serde(rename = "black")]
    NSFW,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub enum RelationType {
    #[serde(rename = "sequel")]
    SEQUEL,
    #[serde(rename = "prequel")]
    PREQUEL,
    #[serde(rename = "alternative_setting")]
    ALTERNATIVESETTING,
    #[serde(rename = "alternative_version")]
    ALTERNATIVEVERSION,
    #[serde(rename = "side_story")]
    SIDESTORY,
    #[serde(rename = "parent_story")]
    PARENTSTORY,
    #[serde(rename = "summary")]
    SUMMARY,
    #[serde(rename = "full_story")]
    FULLSTORY,
}

#[cfg(test)]
mod tests {
    use super::*;
}

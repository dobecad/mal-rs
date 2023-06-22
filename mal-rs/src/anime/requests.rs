// Structs for crafting Anime Endpoint requests
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::{responses::AnimeFieldsEnum, error::AnimeApiError};

#[derive(Debug, Serialize)]
pub struct GetAnimeList {
    q: String,
    limit: u8,
    offset: u32,
    fields: String,
}

impl GetAnimeList {
    pub fn new(q: String, limit: u8, offset: u32, fields: AnimeFields) -> Result<Self, AnimeApiError> {
        if limit > 100 || limit < 1 {
            return Err(AnimeApiError::new("Limit must be between 1 and 100 inclusive".to_string()));
        }

        Ok(Self {
            q,
            limit,
            offset,
            fields: fields.into(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GetAnimeDetails {
    pub(crate) anime_id: u32,
    fields: String, // TODO: Create Enum for fields?
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RankingType {
    #[serde(rename = "all")]
    ALL,
    #[serde(rename = "airing")]
    AIRING,
    #[serde(rename = "upcoming")]
    UPCOMING,
    #[serde(rename = "tv")]
    TV,
    #[serde(rename = "ova")]
    OVA,
    #[serde(rename = "movie")]
    MOVIE,
    #[serde(rename = "special")]
    SPECIAL,
    #[serde(rename = "bypopularity")]
    BYPOPULARITY,
    #[serde(rename = "favorite")]
    FAVORITE,
}

#[derive(Debug, Serialize)]
pub struct GetAnimeRanking {
    ranking_type: RankingType,
    limit: u16,
    offset: u32,
    fields: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Season {
    #[serde(rename = "winter")]
    WINTER,
    #[serde(rename = "spring")]
    SPRING,
    #[serde(rename = "summer")]
    SUMMER,
    #[serde(rename = "fall")]
    FALL,
}

#[derive(Debug, Serialize)]
pub enum SeasonalAnimeSort {
    #[serde(rename = "anime_score")]
    ANIMESCORE,
    #[serde(rename = "anime_num_list_users")]
    ANIMENUMLISTUSERS,
}

#[derive(Debug, Serialize)]
pub struct GetSeasonalAnime {
    year: u8,
    season: Season,
    sort: SeasonalAnimeSort,
    limit: u16,
    offset: u32,
    fields: String,
}

#[derive(Debug, Serialize)]
pub struct GetSuggestedAnime {
    limit: u16,
    offset: u32,
    fields: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnimeStatus {
    #[serde(rename = "watching")]
    WATCHING,
    #[serde(rename = "completed")]
    COMPLETED,
    #[serde(rename = "on_hold")]
    ONHOLD,
    #[serde(rename = "dropped")]
    DROPPED,
    #[serde(rename = "plan_to_watch")]
    PLANTOWATCH,
}

#[derive(Debug, Serialize)]
pub enum UserAnimeListSort {
    #[serde(rename = "list_score")]
    LISTSCORE,
    #[serde(rename = "list_updated_at")]
    LISTUPDATEDAT,
    #[serde(rename = "anime_title")]
    ANIMETITLE,
    #[serde(rename = "anime_start_date")]
    ANIMESTARTDATE,
    #[serde(rename = "anime_id")]
    ANIMEID,
}

#[derive(Debug, Serialize)]
pub struct GetUserAnimeList {
    user_name: String,
    status: AnimeStatus,
    sort: UserAnimeListSort,
    limit: u16,
    offset: u32,
}

pub struct AnimeFields(pub Vec<AnimeFieldsEnum>);

impl Into<String> for AnimeFields {
    fn into(self) -> String {
        let result = self
            .0
            .into_iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<String>>()
            .join(",");
        result
    }
}

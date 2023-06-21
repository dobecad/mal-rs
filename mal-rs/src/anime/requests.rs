// Structs for crafting Anime Endpoint requests
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GetAnimeList {
    q: String,
    limit: u8,
    offset: u16,
    fields: String,
}

#[derive(Debug, Serialize)]
pub struct GetAnimeDetails {
    anime_id: u32,
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

// Structs for crafting Anime Endpoint requests
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::common::limit_check;

use super::{error::AnimeApiError, responses::AnimeFieldsEnum};

#[derive(Debug, Serialize)]
pub struct GetAnimeList {
    q: String,
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetAnimeList {
    pub fn new(
        q: String,
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&AnimeFields>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        Ok(Self {
            q,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GetAnimeDetails {
    #[serde(skip_serializing)]
    pub(crate) anime_id: u32,
    fields: Option<String>, // TODO: Create Enum for fields?
}

impl GetAnimeDetails {
    pub fn new(anime_id: u32, fields: Option<&AnimeFields>) -> Self {
        Self {
            anime_id,
            fields: fields.map(|f| f.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RankingType {
    All,
    Airing,
    Upcoming,
    Tv,
    Ova,
    Movie,
    Special,
    ByPopularity,
    Favorite,
}

#[derive(Debug, Serialize)]
pub struct GetAnimeRanking {
    ranking_type: RankingType,
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetAnimeRanking {
    pub fn new(
        ranking_type: RankingType,
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&AnimeFields>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 500).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 500 inclusive".to_string())
        })?;

        Ok(Self {
            ranking_type,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Winter => {
                write!(f, "winter")
            }
            Self::Fall => {
                write!(f, "fall")
            }
            Self::Summer => {
                write!(f, "summer")
            }
            Self::Spring => {
                write!(f, "spring")
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SeasonalAnimeSort {
    AnimeScore,
    AnimeNumListUsers,
}

#[derive(Debug, Serialize)]
pub struct GetSeasonalAnime {
    #[serde(skip_serializing)]
    pub(crate) year: u16,
    #[serde(skip_serializing)]
    pub(crate) season: Season,
    sort: SeasonalAnimeSort,
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetSeasonalAnime {
    pub fn new(
        year: u16,
        season: Season,
        sort: SeasonalAnimeSort,
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&AnimeFields>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 500).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 500 inclusive".to_string())
        })?;

        Ok(Self {
            year,
            season,
            sort,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GetSuggestedAnime {
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetSuggestedAnime {
    pub fn new(
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&AnimeFields>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        Ok(Self {
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserAnimeListStatus {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserAnimeListSort {
    ListScore,
    ListUpdatedAt,
    AnimeTitle,
    AnimeStartDate,
    // TODO: This sort option is still under development according to MAL API reference
    // AnimeId,
}

#[derive(Debug, Serialize)]
pub struct GetUserAnimeList {
    #[serde(skip_serializing)]
    pub(crate) user_name: String,
    status: UserAnimeListStatus,
    sort: UserAnimeListSort,
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetUserAnimeList {
    /// Note: `user_name` should be the targets user name, or `@me` as a shortcut for yourself
    pub fn new(
        user_name: String,
        status: UserAnimeListStatus,
        sort: UserAnimeListSort,
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&AnimeFields>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 1000).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 1000 inclusive".to_string())
        })?;

        Ok(Self {
            user_name,
            status,
            sort,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateMyAnimeListStatus {
    #[serde(skip_serializing)]
    pub(crate) anime_id: u32,
    status: UserAnimeListStatus,
    is_rewatching: bool,
    score: u8,
    num_watched_episodes: u32,
    priority: u8,
    num_times_rewatched: u32,
    rewatch_value: u8,
    tags: String,
    comments: String,
}

impl UpdateMyAnimeListStatus {
    pub fn new(
        anime_id: u32,
        status: UserAnimeListStatus,
        is_rewatching: bool,
        score: u8,
        num_watched_episodes: u32,
        priority: u8,
        num_times_rewatched: u32,
        rewatch_value: u8,
        tags: String,
        comments: String,
    ) -> Result<Self, AnimeApiError> {
        if score > 10 {
            return Err(AnimeApiError::new(
                "Score must be between 0 and 10 inclusive".to_string(),
            ));
        }
        if priority > 2 {
            return Err(AnimeApiError::new(
                "Priority must be between 0 and 2 inclusive".to_string(),
            ));
        }
        if rewatch_value > 5 {
            return Err(AnimeApiError::new(
                "Rewatch value must be between 0 and 5 inclusive".to_string(),
            ));
        }

        Ok(Self {
            anime_id,
            status,
            is_rewatching,
            score,
            num_watched_episodes,
            priority,
            num_times_rewatched,
            rewatch_value,
            tags,
            comments,
        })
    }
}

#[derive(Debug)]
pub struct DeleteMyAnimeListItem {
    pub(crate) anime_id: u32,
}

impl DeleteMyAnimeListItem {
    pub fn new(anime_id: u32) -> Self {
        Self { anime_id }
    }
}

#[derive(Debug)]
pub struct AnimeFields(pub Vec<AnimeFieldsEnum>);

impl<'a> Into<String> for &'a AnimeFields {
    fn into(self) -> String {
        let result = self
            .0
            .iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<String>>()
            .join(",");
        result
    }
}

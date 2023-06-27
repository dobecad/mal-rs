// Structs for crafting Anime Endpoint requests
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::{error::AnimeApiError, responses::AnimeFieldsEnum};

#[derive(Debug, Serialize)]
pub struct GetAnimeList {
    q: String,
    limit: u8,
    offset: u32,
    fields: String,
}

impl GetAnimeList {
    pub fn new(
        q: String,
        limit: u8,
        offset: u32,
        fields: &AnimeFields,
    ) -> Result<Self, AnimeApiError> {
        if limit > 100 || limit < 1 {
            return Err(AnimeApiError::new(
                "Limit must be between 1 and 100 inclusive".to_string(),
            ));
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
    #[serde(skip_serializing)]
    pub(crate) anime_id: u32,
    fields: String, // TODO: Create Enum for fields?
}

impl GetAnimeDetails {
    pub fn new(anime_id: u32, fields: &AnimeFields) -> Self {
        Self {
            anime_id,
            fields: fields.into(),
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
    fields: String,
}

impl GetAnimeRanking {
    pub fn new(
        ranking_type: RankingType,
        limit: u16,
        offset: u32,
        fields: &AnimeFields,
    ) -> Result<Self, AnimeApiError> {
        if limit < 1 || limit > 500 {
            return Err(AnimeApiError::new(
                "Limit must be between 1 and 500 inclusive".to_string(),
            ));
        }

        Ok(Self {
            ranking_type,
            limit,
            offset,
            fields: fields.into(),
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
    fields: String,
}

impl GetSeasonalAnime {
    pub fn new(
        year: u16,
        season: Season,
        sort: SeasonalAnimeSort,
        limit: u16,
        offset: u32,
        fields: &AnimeFields,
    ) -> Result<Self, AnimeApiError> {
        if limit < 1 || limit > 500 {
            return Err(AnimeApiError::new(
                "Limit must be between 1 and 500 inclusive".to_string(),
            ));
        }

        Ok(Self {
            year,
            season,
            sort,
            limit,
            offset,
            fields: fields.into(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GetSuggestedAnime {
    limit: u16,
    offset: u32,
    fields: String,
}

impl GetSuggestedAnime {
    pub fn new(limit: u16, offset: u32, fields: &AnimeFields) -> Result<Self, AnimeApiError> {
        if limit < 1 || limit > 100 {
            return Err(AnimeApiError::new(
                "Limit must be between 1 and 100 inclusive".to_string(),
            ));
        }

        Ok(Self {
            limit,
            offset,
            fields: fields.into(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AnimeStatus {
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
    status: AnimeStatus,
    sort: UserAnimeListSort,
    limit: u16,
    offset: u32,
    fields: String,
}

impl GetUserAnimeList {
    /// Note: `user_name` should be the targets user name, or `@me` as a shortcut for yourself
    pub fn new(
        user_name: String,
        status: AnimeStatus,
        sort: UserAnimeListSort,
        limit: u16,
        offset: u32,
        fields: &AnimeFields,
    ) -> Result<Self, AnimeApiError> {
        if limit < 1 || limit > 1000 {
            return Err(AnimeApiError::new(
                "Limit must be between 1 and 1000 inclusive".to_string(),
            ));
        }

        Ok(Self {
            user_name,
            status,
            sort,
            limit,
            offset,
            fields: fields.into(),
        })
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

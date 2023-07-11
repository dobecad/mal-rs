use serde::{Deserialize, Serialize};

use crate::common::limit_check;

use super::{error::AnimeApiError, responses::AnimeFieldsEnum};

/// Corresponds to the [Get anime list](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetAnimeList {
    q: String,
    nsfw: bool,
    limit: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetAnimeList {
    /// Create new `Get anime list` query
    ///
    /// Limit must be within `[1, 100]`
    pub fn new(
        q: String,
        nsfw: bool,
        fields: Option<&AnimeFields>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        if q.is_empty() {
            return Err(AnimeApiError::new("Query cannot be empty".to_string()));
        }

        Ok(Self {
            q,
            nsfw,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

/// Corresponds to the [Get anime details](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetAnimeDetails {
    #[serde(skip_serializing)]
    pub(crate) anime_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetAnimeDetails {
    /// Create new `Get anime details` query
    pub fn new(anime_id: u32, fields: Option<&AnimeFields>) -> Self {
        Self {
            anime_id,
            fields: fields.map(|f| f.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

/// Corresponds to the [Get anime ranking](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_ranking_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetAnimeRanking {
    ranking_type: RankingType,
    nsfw: bool,
    limit: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetAnimeRanking {
    /// Create a new `Get anime ranking` query
    ///
    /// Limit must be within `[1, 500]`
    pub fn new(
        ranking_type: RankingType,
        nsfw: bool,
        fields: Option<&AnimeFields>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 500).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 500 inclusive".to_string())
        })?;

        Ok(Self {
            ranking_type,
            nsfw,
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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SeasonalAnimeSort {
    AnimeScore,
    AnimeNumListUsers,
}

/// Corresponds to the [Get seasonal anime](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_season_year_season_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetSeasonalAnime {
    #[serde(skip_serializing)]
    pub(crate) year: u16,
    #[serde(skip_serializing)]
    pub(crate) season: Season,
    nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SeasonalAnimeSort>,
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetSeasonalAnime {
    /// Create a new `Get seasonal anime` query
    ///
    /// Limit must be within `[1, 500]`
    pub fn new(
        year: u16,
        season: Season,
        nsfw: bool,
        fields: Option<&AnimeFields>,
        sort: Option<SeasonalAnimeSort>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 500).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 500 inclusive".to_string())
        })?;

        Ok(Self {
            year,
            season,
            nsfw,
            sort,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

/// Corresponds to the [Get suggested anime](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_suggestions_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetSuggestedAnime {
    nsfw: bool,
    limit: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetSuggestedAnime {
    /// Create a new `Get suggested anime` query
    ///
    /// Limit must be within `[1, 100]`
    pub fn new(
        nsfw: bool,
        fields: Option<&AnimeFields>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        Ok(Self {
            nsfw,
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

/// Corresponds to the [Get user anime list](https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_animelist_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetUserAnimeList {
    #[serde(skip_serializing)]
    pub(crate) user_name: String,
    nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<UserAnimeListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<UserAnimeListSort>,
    limit: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetUserAnimeList {
    /// Create a new `Get user anime list` query
    ///
    /// Limit must be within `[1, 1000]`
    ///
    /// Note: `user_name` should be the targets user name, or `@me` as a
    /// shortcut for yourself. However, you can only use `@me` if you
    /// have an `Oauth` client
    pub fn new(
        user_name: String,
        nsfw: bool,
        fields: Option<&AnimeFields>,
        status: Option<UserAnimeListStatus>,
        sort: Option<UserAnimeListSort>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, AnimeApiError> {
        limit_check(limit, 1, 1000).map_err(|_| {
            AnimeApiError::new("Limit must be between 1 and 1000 inclusive".to_string())
        })?;

        if user_name.is_empty() {
            return Err(AnimeApiError::new("user_name cannot be empty".to_string()));
        }

        Ok(Self {
            user_name,
            nsfw,
            status,
            sort,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            fields: fields.map(|f| f.into()),
        })
    }
}

/// Corresponds to the [Update my anime list status](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_put) endpoint
#[derive(Debug, Serialize)]
pub struct UpdateMyAnimeListStatus {
    #[serde(skip_serializing)]
    pub(crate) anime_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<UserAnimeListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_rewatching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_watched_episodes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_times_rewatched: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rewatch_value: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<String>,
}

impl UpdateMyAnimeListStatus {
    /// Create new `Update my anime list status` query
    ///
    /// Score must be within `[0, 10]`
    ///
    /// Priority must be within `[0, 2]`
    ///
    /// Rewatch_value must be within `[0, 5]`
    pub fn new(
        anime_id: u32,
        status: Option<UserAnimeListStatus>,
        is_rewatching: Option<bool>,
        score: Option<u8>,
        num_watched_episodes: Option<u32>,
        priority: Option<u8>,
        num_times_rewatched: Option<u32>,
        rewatch_value: Option<u8>,
        tags: Option<String>,
        comments: Option<String>,
    ) -> Result<Self, AnimeApiError> {
        if let Some(score) = score {
            if score > 10 {
                return Err(AnimeApiError::new(
                    "Score must be between 0 and 10 inclusive".to_string(),
                ));
            }
        }
        if let Some(priority) = priority {
            if priority > 2 {
                return Err(AnimeApiError::new(
                    "Priority must be between 0 and 2 inclusive".to_string(),
                ));
            }
        }
        if let Some(rewatch_value) = rewatch_value {
            if rewatch_value > 5 {
                return Err(AnimeApiError::new(
                    "Rewatch value must be between 0 and 5 inclusive".to_string(),
                ));
            }
        }

        // TODO: Abstract this logic to make it re-useable
        if !(status.is_some()
            || is_rewatching.is_some()
            || score.is_some()
            || num_watched_episodes.is_some()
            || priority.is_some()
            || num_times_rewatched.is_some()
            || rewatch_value.is_some()
            || tags.is_some()
            || comments.is_some())
        {
            return Err(AnimeApiError::new(
                "At least one of the optional arguments must be Some".to_string(),
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

/// Corresponds to the [Delete my anime list item](https://myanimelist.net/apiconfig/references/api/v2#operation/anime_anime_id_my_list_status_delete) endpoint
#[derive(Debug)]
pub struct DeleteMyAnimeListItem {
    pub(crate) anime_id: u32,
}

impl DeleteMyAnimeListItem {
    /// Create new `Delete my anime list item` query
    pub fn new(anime_id: u32) -> Self {
        Self { anime_id }
    }
}

/// Wrapper for a vector of valid Anime Fields
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anime::all_fields;

    #[test]
    fn test_get_anime_list() {
        let fields = all_fields();
        let query = GetAnimeList::new("".to_string(), false, Some(&fields), Some(100), None);
        assert!(query.is_err());

        let query = GetAnimeList::new("one".to_string(), false, Some(&fields), Some(999), None);
        assert!(query.is_err());

        let query = GetAnimeList::new("one".to_string(), false, Some(&fields), Some(0), None);
        assert!(query.is_err());

        let query = GetAnimeList::new("one".to_string(), false, Some(&fields), Some(50), None);
        assert!(query.is_ok());

        let query = GetAnimeList::new("one".to_string(), false, Some(&fields), None, None);
        assert!(query.is_ok());
    }

    #[test]
    fn test_get_anime_ranking() {
        let fields = all_fields();
        let query = GetAnimeRanking::new(RankingType::All, false, Some(&fields), Some(1000), None);
        assert!(query.is_err());

        let query = GetAnimeRanking::new(RankingType::All, false, Some(&fields), Some(0), None);
        assert!(query.is_err());

        let query = GetAnimeRanking::new(RankingType::All, false, Some(&fields), Some(100), None);
        assert!(query.is_ok());

        let query = GetAnimeRanking::new(RankingType::All, false, Some(&fields), None, None);
        assert!(query.is_ok());
    }

    #[test]
    fn test_get_seasonal_anime() {
        let fields = all_fields();
        let query = GetSeasonalAnime::new(
            1000,
            Season::Spring,
            false,
            Some(&fields),
            Some(SeasonalAnimeSort::AnimeScore),
            Some(999),
            None,
        );
        assert!(query.is_err());

        let query = GetSeasonalAnime::new(
            1000,
            Season::Spring,
            false,
            Some(&fields),
            Some(SeasonalAnimeSort::AnimeScore),
            Some(0),
            None,
        );
        assert!(query.is_err());

        let query = GetSeasonalAnime::new(
            1000,
            Season::Spring,
            false,
            Some(&fields),
            Some(SeasonalAnimeSort::AnimeScore),
            Some(500),
            None,
        );
        assert!(query.is_ok());

        let query = GetSeasonalAnime::new(
            1000,
            Season::Spring,
            false,
            Some(&fields),
            Some(SeasonalAnimeSort::AnimeScore),
            None,
            None,
        );
        assert!(query.is_ok());
    }

    #[test]
    fn test_get_suggested_anime() {
        let fields = all_fields();
        let query = GetSuggestedAnime::new(false, Some(&fields), Some(500), None);
        assert!(query.is_err());

        let query = GetSuggestedAnime::new(false, Some(&fields), Some(0), None);
        assert!(query.is_err());

        let query = GetSuggestedAnime::new(false, Some(&fields), Some(1), None);
        assert!(query.is_ok());

        let query = GetSuggestedAnime::new(false, Some(&fields), None, None);
        assert!(query.is_ok());
    }

    #[test]
    fn test_get_user_anime_list() {
        let fields = all_fields();
        let query = GetUserAnimeList::new(
            "".to_string(),
            false,
            Some(&fields),
            Some(UserAnimeListStatus::Completed),
            Some(UserAnimeListSort::AnimeTitle),
            Some(1001),
            None,
        );
        assert!(query.is_err());

        let query = GetUserAnimeList::new(
            "hello".to_string(),
            false,
            Some(&fields),
            Some(UserAnimeListStatus::Completed),
            Some(UserAnimeListSort::AnimeTitle),
            Some(0),
            None,
        );
        assert!(query.is_err());

        let query = GetUserAnimeList::new(
            "hello".to_string(),
            false,
            Some(&fields),
            Some(UserAnimeListStatus::Completed),
            Some(UserAnimeListSort::AnimeTitle),
            Some(1000),
            None,
        );
        assert!(query.is_ok());

        let query = GetUserAnimeList::new(
            "hello".to_string(),
            false,
            Some(&fields),
            Some(UserAnimeListStatus::Completed),
            Some(UserAnimeListSort::AnimeTitle),
            None,
            None,
        );
        assert!(query.is_ok());
    }

    #[test]
    fn test_update_my_anime_list() {
        let query = UpdateMyAnimeListStatus::new(
            1234, None, None, None, None, None, None, None, None, None,
        );
        assert!(query.is_err());

        let query = UpdateMyAnimeListStatus::new(
            1234,
            Some(UserAnimeListStatus::Dropped),
            None,
            Some(11),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(query.is_err());

        let query = UpdateMyAnimeListStatus::new(
            1234,
            Some(UserAnimeListStatus::Dropped),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            None,
        );
        assert!(query.is_err());

        let query = UpdateMyAnimeListStatus::new(
            1234,
            Some(UserAnimeListStatus::Dropped),
            None,
            None,
            None,
            None,
            None,
            Some(6),
            None,
            None,
        );
        assert!(query.is_err());

        let query = UpdateMyAnimeListStatus::new(
            1234,
            Some(UserAnimeListStatus::Completed),
            None,
            Some(10),
            None,
            Some(2),
            None,
            Some(5),
            None,
            None,
        );
        assert!(query.is_ok());
    }
}

use crate::common::limit_check;

use super::error::MangaApiError;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Serialize)]
pub struct GetMangaList {
    q: String,
    nsfw: bool,
    limit: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetMangaList {
    /// Create new `Get manga list` query
    ///
    /// Limit must be within `[1, 100]`
    pub fn new(
        q: String,
        nsfw: bool,
        fields: Option<&MangaFields>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, MangaApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            MangaApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        if q.is_empty() {
            return Err(MangaApiError::new("Query cannot be empty".to_string()));
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

#[derive(Debug, Serialize)]
pub struct GetMangaDetails {
    #[serde(skip_serializing)]
    pub(crate) manga_id: u32,
    nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetMangaDetails {
    /// Create new `Get manga details` query
    pub fn new(manga_id: u32, nsfw: bool, fields: Option<&MangaDetails>) -> Self {
        Self {
            manga_id,
            nsfw,
            fields: fields.map(|f| f.into()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MangaRankingType {
    All,
    Manga,
    Novels,
    Oneshots,
    Doujin,
    Manhwa,
    Manhua,
    ByPopularity,
    Favorite,
}

#[derive(Debug, Serialize)]
pub struct GetMangaRanking {
    ranking_type: MangaRankingType,
    nsfw: bool,
    limit: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetMangaRanking {
    /// Create new `Get manga ranking`
    ///
    /// Limit must be within `[1, 500]`
    pub fn new(
        ranking_type: MangaRankingType,
        nsfw: bool,
        fields: Option<&MangaFields>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, MangaApiError> {
        limit_check(limit, 1, 500).map_err(|_| {
            MangaApiError::new("Limit must be between 1 and 500 inclusive".to_string())
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserMangaListStatus {
    Reading,
    Completed,
    OnHold,
    Dropped,
    PlanToRead,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserMangaListSort {
    ListScore,
    ListUpdatedAt,
    MangaTitle,
    MangaStartDate,
    // TODO: This sort option is still under development according to MAL API reference
    // MangaId,
}

#[derive(Debug, Serialize)]
pub struct GetUserMangaList {
    #[serde(skip_serializing)]
    pub(crate) user_name: String,
    nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<UserMangaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<UserMangaListSort>,
    limit: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetUserMangaList {
    /// Create new `Get user manga list` query
    ///
    /// Limit must be within `[1, 1000]`
    pub fn new(
        user_name: String,
        nsfw: bool,
        fields: Option<&MangaFields>,
        status: Option<UserMangaListStatus>,
        sort: Option<UserMangaListSort>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, MangaApiError> {
        limit_check(limit, 1, 1000).map_err(|_| {
            MangaApiError::new("Limit must be between 1 and 1000 inclusive".to_string())
        })?;

        if user_name.is_empty() {
            return Err(MangaApiError::new("user_name cannot be empty".to_string()));
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

#[derive(Debug, Serialize)]
pub struct UpdateMyMangaListStatus {
    #[serde(skip_serializing)]
    pub(crate) manga_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<UserMangaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_rereading: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_volumes_read: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_chapters_read: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_times_reread: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reread_value: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<String>,
}

impl UpdateMyMangaListStatus {
    /// Create new `Update my manga list status` query
    ///
    /// Score must be within `[0-10]`
    ///
    /// Priority must be within `[0, 2]`
    ///
    /// Reread_value must be within `[0, 5]`
    pub fn new(
        manga_id: u32,
        status: Option<UserMangaListStatus>,
        is_rereading: Option<bool>,
        score: Option<u8>,
        num_volumes_read: Option<u32>,
        num_chapters_read: Option<u32>,
        priority: Option<u8>,
        num_times_reread: Option<u32>,
        reread_value: Option<u8>,
        tags: Option<String>,
        comments: Option<String>,
    ) -> Result<Self, MangaApiError> {
        if let Some(score) = score {
            if score > 10 {
                return Err(MangaApiError::new(
                    "Score must be between 0 and 10 inclusive".to_string(),
                ));
            }
        }
        if let Some(priority) = priority {
            if priority > 2 {
                return Err(MangaApiError::new(
                    "Priority must be between 0 and 2 inclusive".to_string(),
                ));
            }
        }
        if let Some(reread_value) = reread_value {
            if reread_value > 5 {
                return Err(MangaApiError::new(
                    "Reread value must be between 0 and 5 inclusive".to_string(),
                ));
            }
        }

        if !(status.is_some()
            || is_rereading.is_some()
            || score.is_some()
            || num_chapters_read.is_some()
            || num_volumes_read.is_some()
            || priority.is_some()
            || num_times_reread.is_some()
            || reread_value.is_some()
            || tags.is_some()
            || comments.is_some())
        {
            return Err(MangaApiError::new(
                "At least one of the optional arguments must be Some".to_string(),
            ));
        }

        Ok(Self {
            manga_id,
            status,
            is_rereading,
            score,
            num_volumes_read,
            num_chapters_read,
            priority,
            num_times_reread,
            reread_value,
            tags,
            comments,
        })
    }
}

#[derive(Debug)]
pub struct DeleteMyMangaListItem {
    pub(crate) manga_id: u32,
}

impl DeleteMyMangaListItem {
    /// Create new `Delete my manga list item` query
    pub fn new(manga_id: u32) -> Self {
        Self { manga_id }
    }
}

#[derive(Debug, EnumIter, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MangaField {
    id,
    title,
    main_picture,
    alternative_titles,
    start_date,
    end_date,
    synopsis,
    mean,
    rank,
    popularity,
    num_list_users,
    num_scoring_users,
    nsfw,
    genres,
    created_at,
    updated_at,
    media_type,
    status,
    my_list_status,
    num_volumes,
    num_chapters,
    authors,
}

#[derive(Debug, EnumIter, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MangaDetail {
    // Common fields
    id,
    title,
    main_picture,
    alternative_titles,
    start_date,
    end_date,
    synopsis,
    mean,
    rank,
    popularity,
    num_list_users,
    num_scoring_users,
    nsfw,
    genres,
    created_at,
    updated_at,
    media_type,
    status,
    my_list_status,
    num_volumes,
    num_chapters,
    authors,

    // Detail specific fields
    pictures,
    background,
    related_anime,
    related_manga,
    recommendations,
    serialization,
}

/// Wrapper for a vector of valid Manga Fields
#[derive(Debug)]
pub struct MangaFields(pub Vec<MangaField>);

/// Wrapper for a vector of valid Manga Detail Fields
#[derive(Debug)]
pub struct MangaDetails(pub Vec<MangaDetail>);

impl Into<String> for &MangaFields {
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

impl Into<String> for &MangaDetails {
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
    use crate::manga::all_common_fields;

    #[test]
    fn test_get_manga_list() {
        let fields = all_common_fields();
        let query = GetMangaList::new("".to_string(), false, Some(&fields), None, None);
        assert!(query.is_err());

        let query = GetMangaList::new("one".to_string(), false, Some(&fields), Some(101), None);
        assert!(query.is_err());

        let query = GetMangaList::new("".to_string(), false, Some(&fields), Some(0), None);
        assert!(query.is_err());

        let query = GetMangaList::new("".to_string(), false, Some(&fields), Some(100), None);
        assert!(query.is_err());

        let query = GetMangaList::new("".to_string(), false, Some(&fields), None, None);
        assert!(query.is_err());
    }

    #[test]
    fn test_get_manga_ranking() {
        let fields = all_common_fields();
        let query =
            GetMangaRanking::new(MangaRankingType::All, false, Some(&fields), Some(501), None);
        assert!(query.is_err());

        let query =
            GetMangaRanking::new(MangaRankingType::All, false, Some(&fields), Some(0), None);
        assert!(query.is_err());

        let query =
            GetMangaRanking::new(MangaRankingType::All, false, Some(&fields), Some(500), None);
        assert!(query.is_ok());

        let query = GetMangaRanking::new(MangaRankingType::All, false, Some(&fields), None, None);
        assert!(query.is_ok());
    }

    #[test]
    fn test_get_user_manga_list() {
        let fields = all_common_fields();
        let query =
            GetUserMangaList::new("".to_string(), false, Some(&fields), None, None, None, None);
        assert!(query.is_err());

        let query = GetUserMangaList::new(
            "hello".to_string(),
            false,
            Some(&fields),
            None,
            None,
            Some(1001),
            None,
        );
        assert!(query.is_err());

        let query = GetUserMangaList::new(
            "hello".to_string(),
            false,
            Some(&fields),
            None,
            None,
            Some(0),
            None,
        );
        assert!(query.is_err());

        let query = GetUserMangaList::new(
            "hello".to_string(),
            false,
            Some(&fields),
            None,
            None,
            Some(1000),
            None,
        );
        assert!(query.is_ok());

        let query = GetUserMangaList::new(
            "hello".to_string(),
            false,
            Some(&fields),
            None,
            None,
            None,
            None,
        );
        assert!(query.is_ok());
    }

    #[test]
    fn test_update_my_manga_list_status() {
        let query = UpdateMyMangaListStatus::new(
            1234, None, None, None, None, None, None, None, None, None, None,
        );
        assert!(query.is_err());

        let query = UpdateMyMangaListStatus::new(
            1234,
            Some(UserMangaListStatus::Completed),
            None,
            Some(11),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(query.is_err());

        let query = UpdateMyMangaListStatus::new(
            1234,
            Some(UserMangaListStatus::Completed),
            None,
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

        let query = UpdateMyMangaListStatus::new(
            1234,
            Some(UserMangaListStatus::Completed),
            None,
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

        let query = UpdateMyMangaListStatus::new(
            1234,
            Some(UserMangaListStatus::Completed),
            None,
            Some(10),
            None,
            None,
            Some(2),
            None,
            Some(5),
            None,
            None,
        );
        assert!(query.is_ok())
    }
}

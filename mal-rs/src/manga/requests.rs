use crate::common::limit_check;

use super::{error::MangaApiError, responses::MangaFieldsEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GetMangaList {
    q: String,
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
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&MangaFields>,
    ) -> Result<Self, MangaApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            MangaApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        if q.is_empty() {
            return Err(MangaApiError::new("Query cannot be empty".to_string()));
        }

        Ok(Self {
            q,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetMangaDetails {
    /// Create new `Get manga details` query
    pub fn new(manga_id: u32, fields: Option<&MangaFields>) -> Self {
        Self {
            manga_id,
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
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&MangaFields>,
    ) -> Result<Self, MangaApiError> {
        limit_check(limit, 1, 500).map_err(|_| {
            MangaApiError::new("Limit must be between 1 and 500 inclusive".to_string())
        })?;

        Ok(Self {
            ranking_type,
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
        status: Option<UserMangaListStatus>,
        sort: Option<UserMangaListSort>,
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&MangaFields>,
    ) -> Result<Self, MangaApiError> {
        limit_check(limit, 1, 1000).map_err(|_| {
            MangaApiError::new("Limit must be between 1 and 1000 inclusive".to_string())
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

pub struct MangaFields(pub Vec<MangaFieldsEnum>);

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

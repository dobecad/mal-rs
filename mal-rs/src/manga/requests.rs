use crate::common::limit_check;

// Structs for crafting Manga Endpoint requests
use super::{error::MangaApiError, responses::MangaFieldsEnum};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct GetMangaList {
    q: String,
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetMangaList {
    pub fn new(
        q: String,
        limit: Option<u16>,
        offset: Option<u32>,
        fields: Option<&MangaFields>,
    ) -> Result<Self, MangaApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            MangaApiError::new("Limit must be between 1 and 100 inclusive".to_string())
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
pub struct GetMangaDetails {
    pub(crate) manga_id: u32,
    fields: Option<String>,
}

impl GetMangaDetails {
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
    fields: Option<String>,
}

impl GetMangaRanking {
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
    status: UserMangaListStatus,
    sort: UserMangaListSort,
    limit: u16,
    offset: u32,
    fields: Option<String>,
}

impl GetUserMangaList {
    pub fn new(
        user_name: String,
        status: UserMangaListStatus,
        sort: UserMangaListSort,
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
    status: UserMangaListStatus,
    is_rereading: bool,
    score: u8,
    num_volumes_read: u32,
    num_chapters_read: u32,
    priority: u8,
    num_times_reread: u32,
    reread_value: u8,
    tags: String,
    comments: String,
}

impl UpdateMyMangaListStatus {
    pub fn new(
        manga_id: u32,
        status: UserMangaListStatus,
        is_rereading: bool,
        score: u8,
        num_volumes_read: u32,
        num_chapters_read: u32,
        priority: u8,
        num_times_reread: u32,
        reread_value: u8,
        tags: String,
        comments: String,
    ) -> Result<Self, MangaApiError> {
        if score > 10 {
            return Err(MangaApiError::new(
                "Score must be between 0 and 10 inclusive".to_string(),
            ));
        }
        if priority > 2 {
            return Err(MangaApiError::new(
                "Priority must be between 0 and 2 inclusive".to_string(),
            ));
        }
        if reread_value > 5 {
            return Err(MangaApiError::new(
                "Reread value must be between 0 and 5 inclusive".to_string(),
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

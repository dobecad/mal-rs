// Structs for crafting Manga Endpoint requests
use super::{error::MangaApiError, responses::MangaFieldsEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GetMangaList {
    q: String,
    limit: u8,
    offset: u32,
    fields: String,
}

impl GetMangaList {
    pub fn new(
        q: String,
        limit: u8,
        offset: u32,
        fields: &MangaFields,
    ) -> Result<Self, MangaApiError> {
        if limit > 100 || limit < 1 {
            return Err(MangaApiError::new(
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
pub struct GetMangaDetails {
    pub(crate) manga_id: u32,
    fields: String,
}

impl GetMangaDetails {
    pub fn new(manga_id: u32, fields: &MangaFields) -> Self {
        Self {
            manga_id,
            fields: fields.into(),
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
    fields: String,
}

impl GetMangaRanking {
    pub fn new(
        ranking_type: MangaRankingType,
        limit: u16,
        offset: u32,
        fields: &MangaFields,
    ) -> Result<Self, MangaApiError> {
        if limit < 1 || limit > 500 {
            return Err(MangaApiError::new(
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

#[derive(Debug, Serialize)]
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
    fields: String,
}

impl GetUserMangaList {
    pub fn new(
        user_name: String,
        status: UserMangaListStatus,
        sort: UserMangaListSort,
        limit: u16,
        offset: u32,
        fields: &MangaFields,
    ) -> Result<Self, MangaApiError> {
        if limit < 1 || limit > 1000 {
            return Err(MangaApiError::new(
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
        if score < 0 || score > 10 {
            return Err(MangaApiError::new(
                "Score must be between 0 and 10 inclusive".to_string(),
            ));
        }
        if priority < 0 || priority > 2 {
            return Err(MangaApiError::new(
                "Priority must be between 0 and 2 inclusive".to_string(),
            ));
        }
        if reread_value < 0 || reread_value > 5 {
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

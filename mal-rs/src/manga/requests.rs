// Structs for crafting Manga Endpoint requests
use super::{error::MangaApiError, responses::MangaFieldsEnum};
use serde::Serialize;

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
    ALL,
    MANGA,
    NOVELS,
    ONESHOTS,
    DOUJIN,
    MANHWA,
    MANHUA,
    BYPOPULARITY,
    FAVORITE,
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
pub enum UserMangaListStatus {
    READING,
    COMPLETED,
    ONHOLD,
    DROPPED,
    PLANTOREAD,
}

#[derive(Debug, Serialize)]
pub enum UserMangaListSort {
    LISTSCORE,
    LISTUPDATEDAT,
    MANGATITLE,
    MANGASTARTDATE,
    MANGAID,
}

#[derive(Debug, Serialize)]
pub struct GetUserMangaList {
    user_name: String,
    status: UserMangaListStatus,
    sort: UserMangaListSort,
    limit: u16,
    offset: u32,
}

impl GetUserMangaList {
    pub fn new(
        user_name: String,
        status: UserMangaListStatus,
        sort: UserMangaListSort,
        limit: u16,
        offset: u32,
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
        })
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

// Structs for crafting Manga Endpoint requests
use super::{responses::MangaFieldsEnum, error::MangaApiError};

pub struct GetMangaList {
    q: String,
    limit: u8,
    offset: u32,
    fields: String,
}

impl GetMangaList {
    pub fn new(q: String, limit: u8, offset: u32, fields: MangaFields) -> Result<Self, MangaApiError> {
        if limit > 100 || limit < 1 {
            return Err(MangaApiError::new("Limit must be between 1 and 100 inclusive".to_string()));
        }

        Ok(Self {
            q,
            limit,
            offset,
            fields: fields.into(),
        })
    }
}

pub struct GetMangaDetails {
    manga_id: u32,
    fields: String,
}

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

pub struct GetMangaRanking {
    ranking_type: MangaRankingType,
    limit: u16,
    offset: u32,
    fields: String
}

pub enum UserMangaListStatus {
    READING,
    COMPLETED,
    ONHOLD,
    DROPPED,
    PLANTOREAD,
}

pub enum UserMangaListSort {
    LISTSCORE,
    LISTUPDATEDAT,
    MANGATITLE,
    MANGASTARTDATE,
    MANGAID,
}

pub struct GetUserMangaList {
    user_name: String,
    status: UserMangaListStatus,
    sort: UserMangaListSort,
    limit: u16,
    offset: u32,
}

pub struct MangaFields(Vec<MangaFieldsEnum>);

impl Into<String> for MangaFields {
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
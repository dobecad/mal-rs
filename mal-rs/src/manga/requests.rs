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

#[derive(Debug, Serialize)]
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

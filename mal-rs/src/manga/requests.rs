// Structs for crafting Manga Endpoint requests
use serde::Deserialize;

pub struct GetMangaList {
    q: String,
    limit: u16,
    offset: u32,
    fields: String,
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
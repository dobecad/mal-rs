// Structs for deserializing Manga Endpoint responses
#![allow(dead_code)]

use crate::common_fields::{AlternativeTitles, Genre, MainPicture, Paging, RelationType, NSFW};
use serde::Deserialize;
use enum_from_struct::EnumFromStruct;

#[derive(Debug, Deserialize)]
pub struct MangaList {
    data: Vec<MangaListNode>,
    paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct MangaListNode {
    node: MangaFields,
}

// Wrap everything in Options since user controls what fields should be returned
#[derive(Debug, Deserialize, EnumFromStruct)]
pub struct MangaFields {
    id: Option<u32>,
    title: Option<String>,
    main_picture: Option<MainPicture>,
    alternative_titles: Option<AlternativeTitles>,
    start_date: Option<String>,
    end_date: Option<String>,
    synopsis: Option<String>,
    mean: Option<f32>,
    rank: Option<u32>,
    popularity: Option<u32>,
    num_list_users: Option<u32>,
    num_scoring_users: Option<u32>,
    nsfw: Option<NSFW>,
    genres: Option<Vec<Genre>>,
    created_at: Option<String>,
    updated_at: Option<String>,
    media_type: Option<MediaType>,
    status: Option<Status>,
    my_list_status: Option<MyListStatus>,
    num_volumes: Option<u32>,
    num_chapters: Option<u32>,
    authors: Option<Vec<Author>>,
}

#[derive(Debug, Deserialize)]
pub enum MediaType {
    #[serde(rename = "unknown")]
    UNKNOWN,
    #[serde(rename = "manga")]
    MANGA,
    #[serde(rename = "novel")]
    NOVEL,
    #[serde(rename = "oneshot")]
    ONESHOT,
    #[serde(rename = "doujinshi")]
    DOUJINSHI,
    #[serde(rename = "manhwa")]
    MANHWA,
    #[serde(rename = "manhua")]
    MANHUA,
    #[serde(rename = "oel")]
    OEL,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "finished")]
    FINISHED,
    #[serde(rename = "currently_published")]
    CURRENTLYPUBLISHING,
    #[serde(rename = "not_yet_published")]
    NOTYETPUBLISHED,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    node: AuthorDetails,
    role: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorDetails {
    id: u32,
    first_name: String,
    last_name: String,
}

#[derive(Debug, Deserialize)]
pub enum ListStatus {
    #[serde(rename = "reading")]
    READING,
    #[serde(rename = "completed")]
    COMPLETED,
    #[serde(rename = "on_hold")]
    ONHOLD,
    #[serde(rename = "dropped")]
    DROPPED,
    #[serde(rename = "plan_to_read")]
    PLANTOREAD,
}

#[derive(Debug, Deserialize)]
pub struct MyListStatus {
    status: Option<ListStatus>,
    score: u8,
    num_volumes_read: u32,
    num_chapters_read: u32,
    is_rereading: bool,
    start_date: Option<String>,
    finish_date: Option<String>,
    priority: u32,
    num_times_reread: u32,
    reread_value: u32,
    tags: Vec<String>,
    comments: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaPicture {
    medium: String,
    large: String,
}

#[derive(Debug, Deserialize)]
pub struct RelatedManga {
    node: MangaFields,
    relation_type: RelationType,
    relation_type_formatted: String,
}

#[derive(Debug, Deserialize)]
pub struct Recommendation {
    node: MangaFields,
    num_recommendations: u32,
}

#[derive(Debug, Deserialize)]
pub struct Serialization {
    node: SerializationNode,
    role: String,
}

#[derive(Debug, Deserialize)]
pub struct SerializationNode {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaDetails {
    #[serde(flatten)]
    shared_fields: MangaFields,

    pictures: Option<Vec<MangaPicture>>,
    background: Option<String>,
    related_anime: Option<Vec<crate::anime::responses::RelatedAnime>>,
    related_manga: Option<Vec<RelatedManga>>,
    recommendations: Option<Vec<Recommendation>>,
    serialization: Option<Vec<Serialization>>,
}

#[derive(Debug, Deserialize)]
pub struct MangaRanking {
    data: Vec<MangaRankingNode>,
    paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct MangaRankingNode {
    node: MangaFields,
    ranking: Ranking,
}

#[derive(Debug, Deserialize)]
pub struct Ranking {
    rank: u32,
    previous_rank: Option<u32>,
}

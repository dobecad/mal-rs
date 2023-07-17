use std::fmt::Display;

use crate::common::{
    AlternativeTitles, Genre, MainPicture, Paging, PagingIter, RelationType, NSFW,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MangaList {
    pub data: Vec<MangaListNode>,
    pub paging: Paging,
}

impl Display for MangaList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

impl PagingIter for MangaList {
    type Item = Self;

    fn next_page(&self) -> Option<&String> {
        self.paging.next.as_ref()
    }

    fn prev_page(&self) -> Option<&String> {
        self.paging.previous.as_ref()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MangaListNode {
    pub node: MangaFields,

    /// This field is only present when querying for a User's anime list
    pub list_status: Option<ListStatus>,
}

impl Display for MangaListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

// Wrap everything in Options since user controls what fields should be returned
#[derive(Debug, Deserialize, Serialize)]
pub struct MangaFields {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub main_picture: Option<MainPicture>,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub synopsis: Option<String>,
    pub mean: Option<f32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub num_list_users: Option<u32>,
    pub num_scoring_users: Option<u32>,
    pub nsfw: Option<NSFW>,
    pub genres: Option<Vec<Genre>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub media_type: Option<MediaType>,
    pub status: Option<Status>,
    pub my_list_status: Option<ListStatus>,
    pub num_volumes: Option<u32>,
    pub num_chapters: Option<u32>,
    pub authors: Option<Vec<Author>>,
}

impl Display for MangaFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Unknown,
    Manga,
    Novel,
    Oneshot,
    Doujinshi,
    Manhwa,
    Manhua,
    Oel,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Finished,
    CurrentlyPublishing,
    NotYetPublished,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub node: AuthorDetails,
    pub role: Option<String>,
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorDetails {
    pub id: u32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Display for AuthorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListStatus {
    pub status: Option<super::requests::UserMangaListStatus>,
    pub score: u8,
    pub num_volumes_read: u32,
    pub num_chapters_read: u32,
    pub is_rereading: bool,
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub priority: u8,
    pub num_times_reread: u32,
    pub reread_value: u8,
    pub tags: Vec<String>,
    pub comments: String,
    pub updated_at: String,
}

impl Display for ListStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyListStatus {
    pub status: Option<super::requests::UserMangaListStatus>,
    pub is_rereading: bool,
    pub score: u8,
    pub num_volumes_read: u32,
    pub num_chapters_read: u32,
    pub priority: u8,
    pub num_times_reread: u32,
    pub reread_value: u8,
    pub tags: String,
    pub comments: String,
}

impl Display for MyListStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MangaPicture {
    pub medium: String,
    pub large: String,
}

impl Display for MangaPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedManga {
    pub node: MangaFields,
    pub relation_type: RelationType,
    pub relation_type_formatted: String,
}

impl Display for RelatedManga {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recommendation {
    pub node: MangaFields,
    pub num_recommendations: u32,
}

impl Display for Recommendation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Serialization {
    pub node: SerializationNode,
    pub role: Option<String>,
}

impl Display for Serialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerializationNode {
    pub id: u32,
    pub name: String,
}

impl Display for SerializationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MangaDetails {
    #[serde(flatten)]
    pub shared_fields: MangaFields,

    pub pictures: Option<Vec<MangaPicture>>,
    pub background: Option<String>,
    pub related_anime: Option<Vec<crate::anime::responses::RelatedAnime>>,
    pub related_manga: Option<Vec<RelatedManga>>,
    pub recommendations: Option<Vec<Recommendation>>,
    pub serialization: Option<Vec<Serialization>>,
}

impl Display for MangaDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MangaRanking {
    pub data: Vec<MangaRankingNode>,
    pub paging: Paging,
}

impl Display for MangaRanking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

impl PagingIter for MangaRanking {
    type Item = Self;

    fn next_page(&self) -> Option<&String> {
        self.paging.next.as_ref()
    }

    fn prev_page(&self) -> Option<&String> {
        self.paging.previous.as_ref()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MangaRankingNode {
    pub node: MangaFields,
    pub ranking: Ranking,
}

impl Display for MangaRankingNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ranking {
    pub rank: u32,
    pub previous_rank: Option<u32>,
}

impl Display for Ranking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

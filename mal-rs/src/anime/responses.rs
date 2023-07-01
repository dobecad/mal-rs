// Structs for deserializing Anime Endpoint responses

use crate::common::{
    AlternativeTitles, Genre, MainPicture, Paging, PagingIter, RelationType, NSFW,
};
use enum_from_struct::EnumFromStruct;
use serde::Deserialize;

// This is imported for the `enum-from-struct` proc macro
use strum_macros::EnumIter;

#[derive(Debug, Deserialize)]
pub struct AnimeList {
    pub data: Vec<AnimeListNode>,
    pub paging: Paging,
}

impl PagingIter for AnimeList {
    type Item = Self;

    fn next_page(&self) -> &Option<String> {
        &self.paging.next
    }

    fn prev_page(&self) -> &Option<String> {
        &self.paging.previous
    }
}

#[derive(Debug, Deserialize)]
pub struct AnimeListNode {
    pub node: AnimeFields,

    /// This field is only present when querying for a User's anime list
    pub list_status: Option<ListStatus>,
}

#[derive(Debug, Deserialize)]
pub struct AnimePicture {
    pub medium: String,
    pub large: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Unknown,
    Tv,
    Ova,
    Movie,
    Special,
    Ona,
    Music,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    FinishedAiring,
    CurrentlyAiring,
    NotYetAired,
}

#[derive(Debug, Deserialize)]
pub struct ListStatus {
    pub status: Option<super::requests::UserAnimeListStatus>,
    pub score: u8,
    pub num_episodes_watched: u32,
    pub is_rewatching: bool,
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub priority: u8,
    pub num_times_rewatched: u32,
    pub rewatch_value: u8,
    pub tags: Vec<String>,
    pub comments: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct MyListStatus {
    pub status: Option<super::requests::UserAnimeListStatus>,
    pub is_rewatching: bool,
    pub score: u8,
    pub num_watched_episodes: u32,
    pub priority: u8,
    pub num_times_rewatched: u32,
    pub rewatch_value: u8,
    pub tags: String,
    pub comments: String,
}

#[derive(Debug, Deserialize)]
pub struct StartSeason {
    pub year: u32,
    pub season: super::requests::Season,
}

#[derive(Debug, Deserialize)]
pub struct Broadcast {
    pub day_of_the_week: String,
    pub start_time: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Other,
    Original,
    Manga,
    #[serde(rename = "4_koma_manga")]
    KomaManga,
    WebManga,
    DigitalMedia,
    Novel,
    LightNovel,
    VisualNovel,
    Game,
    CardGame,
    Book,
    PictureBook,
    Radio,
    Music,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    G,
    PG,
    #[serde(rename = "pg_13")]
    PG13,
    R,
    #[serde(rename = "r+")]
    RR,
    RX,
}

#[derive(Debug, Deserialize)]
pub struct Studio {
    pub id: u32,
    pub name: String,
}

// Wrap everything in Options since user controls what fields should be returned
#[derive(Debug, Deserialize, EnumFromStruct)]
pub struct AnimeFields {
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
    pub num_episodes: Option<u32>,
    pub start_season: Option<StartSeason>,
    pub broadcast: Option<Broadcast>,
    pub source: Option<Source>,
    pub average_episode_duration: Option<u32>,
    pub rating: Option<Rating>,
    pub studios: Option<Vec<Studio>>,
}

#[derive(Debug, Deserialize)]
pub struct RelatedAnime {
    pub node: AnimeFields,
    pub relation_type: RelationType,
    pub relation_type_formatted: String,
}

#[derive(Debug, Deserialize)]
pub struct Recommendations {
    pub node: AnimeFields,
    pub num_recommendations: u32,
}

#[derive(Debug, Deserialize)]
pub struct Statistics {
    pub num_list_users: u32,
    pub status: StatisticsStatus,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsStatus {
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
}

#[derive(Debug, Deserialize)]
pub struct AnimeDetails {
    #[serde(flatten)]
    pub shared_fields: AnimeFields,

    pub pictures: Option<Vec<AnimePicture>>,
    pub background: Option<String>,
    pub related_anime: Vec<RelatedAnime>,
    pub related_manga: Option<Vec<crate::manga::responses::RelatedManga>>, // TODO: Add this once Manga structs done
    pub recommendations: Vec<Recommendations>,
    pub statistics: Option<Statistics>,
}

#[derive(Debug, Deserialize)]
pub struct Ranking {
    pub rank: u32,
    pub previous_rank: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeRanking {
    pub data: Vec<AnimeRankingNode>,
    pub paging: Paging,
}

impl PagingIter for AnimeRanking {
    type Item = Self;

    fn next_page(&self) -> &Option<String> {
        &self.paging.next
    }

    fn prev_page(&self) -> &Option<String> {
        &self.paging.previous
    }
}

#[derive(Debug, Deserialize)]
pub struct AnimeRankingNode {
    pub node: AnimeFields,
    pub ranking: Ranking,
}

#[derive(Debug, Deserialize)]
pub struct SeasonalAnime {
    pub data: Vec<SeasonalAnimeNode>,
    pub paging: Paging,
}

impl PagingIter for SeasonalAnime {
    type Item = Self;

    fn next_page(&self) -> &Option<String> {
        &self.paging.next
    }

    fn prev_page(&self) -> &Option<String> {
        &self.paging.previous
    }
}

#[derive(Debug, Deserialize)]
pub struct SeasonalAnimeNode {
    pub node: AnimeFields,
}

#[derive(Debug, Deserialize)]
pub struct SuggestedAnime {
    pub data: Vec<SuggestedAnimeNode>,
    pub paging: Paging,
}

impl PagingIter for SuggestedAnime {
    type Item = Self;

    fn next_page(&self) -> &Option<String> {
        &self.paging.next
    }

    fn prev_page(&self) -> &Option<String> {
        &self.paging.previous
    }
}

#[derive(Debug, Deserialize)]
pub struct SuggestedAnimeNode {
    pub node: AnimeFields,
}

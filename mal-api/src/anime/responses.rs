use std::fmt::Display;

use crate::common::{
    AlternativeTitles, Genre, MainPicture, Paging, PagingIter, RelationType, NSFW,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeList {
    pub data: Vec<AnimeListNode>,
    pub paging: Paging,
}

impl PagingIter for AnimeList {
    type Item = Self;

    fn next_page(&self) -> Option<&String> {
        self.paging.next.as_ref()
    }

    fn prev_page(&self) -> Option<&String> {
        self.paging.previous.as_ref()
    }
}

impl Display for AnimeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeListNode {
    pub node: AnimeFields,

    /// This field is only present when querying for a User's anime list
    pub list_status: Option<ListStatus>,
}

impl Display for AnimeListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimePicture {
    pub medium: String,
    pub large: String,
}

impl Display for AnimePicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    FinishedAiring,
    CurrentlyAiring,
    NotYetAired,
}

#[derive(Debug, Deserialize, Serialize)]
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

impl Display for ListStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StartSeason {
    pub year: u32,
    pub season: super::requests::Season,
}

impl Display for StartSeason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Broadcast {
    pub day_of_the_week: String,
    pub start_time: Option<String>,
}

impl Display for Broadcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    G,
    PG,
    #[serde(rename = "pg_13")]
    PG13,
    R,
    #[serde(rename = "r+")]
    RP,
    RX,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Studio {
    pub id: u32,
    pub name: String,
}

impl Display for Studio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

// Wrap everything in Options since user controls what fields should be returned
#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeFields {
    pub id: u32,
    pub title: String,
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

impl Display for AnimeFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedAnime {
    pub node: AnimeFields,
    pub relation_type: RelationType,
    pub relation_type_formatted: String,
}

impl Display for RelatedAnime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recommendations {
    pub node: AnimeFields,
    pub num_recommendations: u32,
}

impl Display for Recommendations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Statistics {
    pub num_list_users: u32,
    pub status: StatisticsStatus,
}

impl Display for Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatisticsStatus {
    // MAL returns these as strings, even though docs say they are supposed to be integers
    // Use custom serializer for these fields to turn the strings into u32
    #[serde(deserialize_with = "deserialize_string_to_u32")]
    pub watching: u32,
    #[serde(deserialize_with = "deserialize_string_to_u32")]
    pub completed: u32,
    #[serde(deserialize_with = "deserialize_string_to_u32")]
    pub on_hold: u32,
    #[serde(deserialize_with = "deserialize_string_to_u32")]
    pub dropped: u32,
    #[serde(deserialize_with = "deserialize_string_to_u32")]
    pub plan_to_watch: u32,
}

impl Display for StatisticsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

fn deserialize_string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    if let Some(number) = value.as_str().and_then(|s| s.parse().ok()) {
        Ok(number)
    } else {
        Err(serde::de::Error::custom("Invalid value for u32"))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeDetails {
    #[serde(flatten)]
    pub shared_fields: AnimeFields,

    pub pictures: Option<Vec<AnimePicture>>,
    pub background: Option<String>,
    pub related_anime: Option<Vec<RelatedAnime>>,
    pub related_manga: Option<Vec<crate::manga::responses::RelatedManga>>,
    pub recommendations: Option<Vec<Recommendations>>,
    pub statistics: Option<Statistics>,
}

impl Display for AnimeDetails {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeRanking {
    pub data: Vec<AnimeRankingNode>,
    pub paging: Paging,
}

impl Display for AnimeRanking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

impl PagingIter for AnimeRanking {
    type Item = Self;

    fn next_page(&self) -> Option<&String> {
        self.paging.next.as_ref()
    }

    fn prev_page(&self) -> Option<&String> {
        self.paging.previous.as_ref()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeRankingNode {
    pub node: AnimeFields,
    pub ranking: Ranking,
}

impl Display for AnimeRankingNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonalAnime {
    pub data: Vec<SeasonalAnimeNode>,
    pub paging: Paging,
}

impl Display for SeasonalAnime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

impl PagingIter for SeasonalAnime {
    type Item = Self;

    fn next_page(&self) -> Option<&String> {
        self.paging.next.as_ref()
    }

    fn prev_page(&self) -> Option<&String> {
        self.paging.previous.as_ref()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonalAnimeNode {
    pub node: AnimeFields,
}

impl Display for SeasonalAnimeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuggestedAnime {
    pub data: Vec<SuggestedAnimeNode>,
    pub paging: Paging,
}

impl Display for SuggestedAnime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

impl PagingIter for SuggestedAnime {
    type Item = Self;

    fn next_page(&self) -> Option<&String> {
        self.paging.next.as_ref()
    }

    fn prev_page(&self) -> Option<&String> {
        self.paging.previous.as_ref()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuggestedAnimeNode {
    pub node: AnimeFields,
}

impl Display for SuggestedAnimeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

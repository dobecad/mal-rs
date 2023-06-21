// Structs for deserializing Anime Endpoint responses
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Paging {
    previous: String,
    next: String,
}

#[derive(Debug, Deserialize)]
pub struct AnimeList {
    data: Vec<AnimeListNode>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeListNode {
    node: AnimeFields,
}

#[derive(Debug, Deserialize)]
pub struct AnimeMainPicture {
    medium: String,
    large: String,
}

#[derive(Debug, Deserialize)]
pub struct AnimePicture {
    medium: String,
    large: String,
}

#[derive(Debug, Deserialize)]
pub struct AlternativeTitles {
    synonyms: Option<Vec<String>>,
    en: Option<String>,
    ja: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum NSFW {
    #[serde(rename = "white")]
    SFW,
    #[serde(rename = "gray")]
    MNSFW,
    #[serde(rename = "black")]
    NSFW,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub enum MediaType {
    #[serde(rename = "unknown")]
    UNKNOWN,
    #[serde(rename = "tv")]
    TV,
    #[serde(rename = "ova")]
    OVA,
    #[serde(rename = "movie")]
    MOVIE,
    #[serde(rename = "special")]
    SPECIAL,
    #[serde(rename = "ona")]
    ONA,
    #[serde(rename = "music")]
    MUSIC,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "finished_airing")]
    FINISHEDAIRING,
    #[serde(rename = "currently_airing")]
    CURRENTLYAIRING,
    #[serde(rename = "not_yet_aired")]
    NOTYETAIRED,
}

#[derive(Debug, Deserialize)]
pub struct MyListStatus {
    status: Option<super::requests::AnimeStatus>,
    score: u8,
    num_episodes_watched: u32,
    is_rewatching: bool,
    start_date: Option<String>,
    finish_date: Option<String>,
    priority: u16,
    num_times_rewatched: u32,
    rewatch_value: u32,
    tags: Vec<String>,
    comments: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct StartSeason {
    year: u32,
    season: super::requests::Season,
}

#[derive(Debug, Deserialize)]
pub struct Broadcast {
    day_of_the_week: String,
    start_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Source {
    #[serde(rename = "other")]
    OTHER,
    #[serde(rename = "original")]
    ORIGINAL,
    #[serde(rename = "manga")]
    MANGA,
    #[serde(rename = "4_koma_manga")]
    KOMAMANGA,
    #[serde(rename = "web_manga")]
    WEBMANGA,
    #[serde(rename = "digital_manga")]
    DIGITALMANGA,
    #[serde(rename = "novel")]
    NOVEL,
    #[serde(rename = "light_novel")]
    LIGHTNOVEL,
    #[serde(rename = "visual_novel")]
    VISUALNOVEL,
    #[serde(rename = "game")]
    GAME,
    #[serde(rename = "card_game")]
    CARDGAME,
    #[serde(rename = "book")]
    BOOK,
    #[serde(rename = "picture_book")]
    PICTUREBOOK,
    #[serde(rename = "radio")]
    RADIO,
    #[serde(rename = "music")]
    MUSIC,
}

#[derive(Debug, Deserialize)]
pub enum Rating {
    #[serde(rename = "g")]
    G,
    #[serde(rename = "pg")]
    PG,
    #[serde(rename = "pg_13")]
    PG13,
    #[serde(rename = "r")]
    R,
    #[serde(rename = "r+")]
    RR,
    #[serde(rename = "rx")]
    RX,
}

#[derive(Debug, Deserialize)]
pub struct Studio {
    id: u32,
    name: String,
}

// Wrap everything in Options since user controls what fields should be returned
#[derive(Debug, Deserialize)]
pub struct AnimeFields {
    id: Option<u32>,
    title: Option<String>,
    main_picture: Option<AnimeMainPicture>,
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
    num_episodes: Option<u32>,
    start_season: Option<StartSeason>,
    broadcast: Option<Broadcast>,
    source: Option<Source>,
    average_episode_duration: Option<u32>,
    rating: Option<Rating>,
    studios: Option<Vec<Studio>>,
}

#[derive(Debug, Deserialize)]
pub enum RelationType {
    #[serde(rename = "sequel")]
    SEQUEL,
    #[serde(rename = "prequel")]
    PREQUEL,
    #[serde(rename = "alternative_setting")]
    ALTERNATIVESETTING,
    #[serde(rename = "alternative_version")]
    ALTERNATIVEVERSION,
    #[serde(rename = "side_story")]
    SIDESTORY,
    #[serde(rename = "parent_story")]
    PARENTSTORY,
    #[serde(rename = "summary")]
    SUMMARY,
    #[serde(rename = "full_story")]
    FULLSTORY,
}

#[derive(Debug, Deserialize)]
pub struct RelatedAnime {
    node: AnimeFields,
    relation_type: RelationType,
    relation_type_formatted: String,
}

#[derive(Debug, Deserialize)]
pub struct Recommendations {
    node: AnimeFields,
    num_recommendations: u32,
}

#[derive(Debug, Deserialize)]
pub struct Statistics {
    num_list_users: u32,
    status: StatisticsStatus,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsStatus {
    watching: u32,
    completed: u32,
    on_hold: u32,
    dropped: u32,
    plan_to_watch: u32,
}

#[derive(Debug, Deserialize)]
pub struct AnimeDetails {
    #[serde(flatten)]
    shared_fields: AnimeFields,

    pictures: Option<Vec<AnimePicture>>,
    background: Option<String>,
    related_anime: Vec<RelatedAnime>,
    related_manga: Option<Vec<String>>, // TODO: Add this once Manga structs done
    recommendations: Vec<Recommendations>,
    statistics: Option<Statistics>,
}

#[derive(Debug, Deserialize)]
pub struct Ranking {
    rank: u32,
    previous_rank: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeRanking {
    #[serde(flatten)]
    shared_fields: AnimeFields,
    ranking: Ranking,
}

#[derive(Debug, Deserialize)]
pub struct SeasonalAnime {
    #[serde(flatten)]
    shared_fields: AnimeFields,
}

#[derive(Debug, Deserialize)]
pub struct SuggestedAnime {
    #[serde(flatten)]
    shared_fields: AnimeFields,
}

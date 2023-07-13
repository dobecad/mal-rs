use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub picture: String,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub location: Option<String>,
    pub joined_at: Option<String>,
    pub anime_statistics: Option<AnimeStatistics>,
    pub time_zone: Option<String>,
    pub is_supporter: bool,
}

#[derive(Debug, EnumIter, PartialEq)]
#[allow(non_camel_case_types)]
pub enum UserFieldsEnum {
    id,
    name,
    picture,
    gender,
    birthday,
    location,
    joined_at,
    anime_statistics,
    time_zone,
    is_supporter,
}

#[derive(Debug, Deserialize)]
pub struct AnimeStatistics {
    num_items_watching: u32,
    num_items_completed: u32,
    num_items_on_hold: u32,
    num_items_dropped: u32,
    num_items_plan_to_watch: u32,
    num_items: u32,
    num_days_watched: f32,
    num_days_watching: f32,
    num_days_completed: f32,
    num_days_on_hold: f32,
    num_days_dropped: f32,
    num_days: f32,
    num_episodes: u32,
    num_times_rewatched: u32,
    mean_score: f32,
}

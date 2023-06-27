// Structs for deserializing User Endpoint responses
#![allow(dead_code)]

use enum_from_struct::EnumFromStruct;
use serde::Deserialize;

// This is imported for the `enum-from-struct` proc macro
use strum_macros::EnumIter;

#[derive(Debug, Deserialize, EnumFromStruct)]
pub struct User {
    id: u32,
    name: String,
    picture: String,
    gender: Option<String>,
    birthday: Option<String>,
    location: Option<String>,
    joined_at: Option<String>,
    anime_statistics: Option<AnimeStatistics>,
    time_zone: Option<String>,
    is_supporter: bool,
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

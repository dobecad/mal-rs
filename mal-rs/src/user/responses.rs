use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeStatistics {
    pub num_items_watching: u32,
    pub num_items_completed: u32,
    pub num_items_on_hold: u32,
    pub num_items_dropped: u32,
    pub num_items_plan_to_watch: u32,
    pub num_items: u32,
    pub num_days_watched: f32,
    pub num_days_watching: f32,
    pub num_days_completed: f32,
    pub num_days_on_hold: f32,
    pub num_days_dropped: f32,
    pub num_days: f32,
    pub num_episodes: u32,
    pub num_times_rewatched: u32,
    pub mean_score: f32,
}

impl Display for AnimeStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

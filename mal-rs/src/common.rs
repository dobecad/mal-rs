use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Paging {
    previous: Option<String>,
    next: String,
}

#[derive(Debug, Deserialize)]
pub struct MainPicture {
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
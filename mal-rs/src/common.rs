//! Module containing common request/response fields, traits, and functions

use std::{
    error::Error,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct CommonError {
    pub message: String,
}

impl Error for CommonError {}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CommonError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paging {
    pub previous: Option<String>,
    pub next: Option<String>,
}

impl Display for Paging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainPicture {
    pub medium: String,
    pub large: String,
}

impl Display for MainPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlternativeTitles {
    pub synonyms: Option<Vec<String>>,
    pub en: Option<String>,
    pub ja: Option<String>,
}

impl Display for AlternativeTitles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NSFW {
    #[serde(rename = "white")]
    SFW,
    #[serde(rename = "gray")]
    MNSFW,
    #[serde(rename = "black")]
    NSFW,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

impl Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Sequel,
    Prequel,
    AlternativeSetting,
    AlternativeVersion,
    SideStory,
    ParentStory,
    Summary,
    FullStory,
}

pub(crate) fn limit_check(
    value: Option<u16>,
    lowerbound: u16,
    upperbound: u16,
) -> Result<(), CommonError> {
    if value.is_some() {
        let value = value.unwrap();
        if value < lowerbound || value > upperbound {
            return Err(CommonError::new("Given limit is out of range".to_string()));
        }
    }
    Ok(())
}

pub trait PagingIter {
    type Item;

    fn next_page(&self) -> Option<&String>;

    fn prev_page(&self) -> Option<&String>;
}

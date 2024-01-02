//! Module containing common request/response fields, traits, and functions

use std::{
    collections::HashMap,
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum NSFW {
    #[serde(rename = "white")]
    SFW,
    #[serde(rename = "gray")]
    MNSFW,
    #[serde(rename = "black")]
    NSFW,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
pub struct Ranking {
    pub rank: u32,
    pub previous_rank: Option<u32>,
}

impl Display for Ranking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
    Character, // this type is not documented in the MAL API reference...
}

pub(crate) fn struct_to_form_data<T>(query: &T) -> Result<HashMap<String, String>, Box<dyn Error>>
where
    T: Serialize,
{
    let form = serde_urlencoded::to_string(&query)?
        .split('&')
        .map(|x| {
            let mut parts = x.splitn(2, "=");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap_or("").to_string();
            (key, value)
        })
        .collect();
    Ok(form)
}

pub trait PagingIter {
    type Item;

    fn next_page(&self) -> Option<&String>;

    fn prev_page(&self) -> Option<&String>;
}

use std::{error::Error, fmt};

use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct Paging {
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MainPicture {
    pub medium: String,
    pub large: String,
}

#[derive(Debug, Deserialize)]
pub struct AlternativeTitles {
    pub synonyms: Option<Vec<String>>,
    pub en: Option<String>,
    pub ja: Option<String>,
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
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
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

pub(crate) fn limit_check(value: Option<u16>, lowerbound: u16, upperbound: u16) -> Result<(), CommonError> {
    if value.is_some() {
        let value = value.unwrap();
        if value < lowerbound || value > upperbound {
            return Err(CommonError::new("Given limit is out of range".to_string()))
        }
    }
    Ok(())
}

pub trait PagingIter {
    type Item;

    fn next_page(&self) -> &Option<String>;

    fn prev_page(&self) -> &Option<String>;
}
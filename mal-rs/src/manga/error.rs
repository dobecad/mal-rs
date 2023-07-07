use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MangaApiError {
    pub message: String,
}

impl Error for MangaApiError {}

impl fmt::Display for MangaApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl MangaApiError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AnimeApiError {
    pub message: String,
}

impl Error for AnimeApiError {}

impl fmt::Display for AnimeApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl AnimeApiError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ForumApiError {
    pub message: String,
}

impl Error for ForumApiError {}

impl fmt::Display for ForumApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ForumApiError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
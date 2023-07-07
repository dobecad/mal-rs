use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UserApiError {
    pub message: String,
}

impl Error for UserApiError {}

impl fmt::Display for UserApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl UserApiError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

//! Module for interacting with the `anime` and `user animelist` endpoints

use self::{requests::AnimeFields, responses::AnimeFieldsEnum};
use strum::IntoEnumIterator;

/// Anime API client
pub mod api;

/// Anime API errors
pub mod error;

/// Anime API request structs
pub mod requests;

/// Anime API responses
pub mod responses;

/// Return all of the possible Anime Fields
pub fn all_fields() -> AnimeFields {
    let mut vec = Vec::with_capacity(AnimeFieldsEnum::iter().len());
    for variant in AnimeFieldsEnum::iter() {
        vec.push(variant);
    }
    AnimeFields(vec)
}

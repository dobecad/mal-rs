//! Module for interacting with the `anime` and `user animelist` endpoints

use self::{requests::AnimeFields, responses::AnimeFieldsEnum};
use strum::IntoEnumIterator;

/// API client
pub mod api;

/// API errors
pub mod error;

/// API request structs
pub mod requests;

/// API responses
pub mod responses;

/// Return all of the possible Anime Fields
pub fn all_fields() -> AnimeFields {
    let mut vec = Vec::with_capacity(AnimeFieldsEnum::iter().len());
    for variant in AnimeFieldsEnum::iter() {
        vec.push(variant);
    }
    AnimeFields(vec)
}

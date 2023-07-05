//! Module for interacting with the `manga` and `user mangalist` endpoints

use self::{requests::MangaFields, responses::MangaFieldsEnum};
use strum::IntoEnumIterator;

/// API client
pub mod api;

/// API errors
pub mod error;

/// API request structs
pub mod requests;

/// API responses
pub mod responses;


/// Return all of the possible Manga Fields
pub fn all_fields() -> MangaFields {
    let mut vec = Vec::with_capacity(MangaFieldsEnum::iter().len());
    for variant in MangaFieldsEnum::iter() {
        vec.push(variant);
    }
    MangaFields(vec)
}

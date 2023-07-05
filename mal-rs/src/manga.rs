//! Module for interacting with the `manga` and `user mangalist` endpoints

use self::{requests::MangaFields, responses::MangaFieldsEnum};
use strum::IntoEnumIterator;

/// Manga API client
pub mod api;

/// Manga API errors
pub mod error;

/// Manga API request structs
pub mod requests;

/// Manga API responses
pub mod responses;


/// Return all of the possible Manga Fields
pub fn all_fields() -> MangaFields {
    let mut vec = Vec::with_capacity(MangaFieldsEnum::iter().len());
    for variant in MangaFieldsEnum::iter() {
        vec.push(variant);
    }
    MangaFields(vec)
}

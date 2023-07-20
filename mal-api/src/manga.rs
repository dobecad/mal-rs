//! Module for interacting with the `manga` and `user mangalist` endpoints

use self::requests::{MangaDetail, MangaDetailFields, MangaField, MangaCommonFields};
use strum::IntoEnumIterator;

/// Manga API client
pub mod api;

/// Manga API errors
pub mod error;

/// Manga API request structs
pub mod requests;

/// Manga API responses
pub mod responses;

/// Return all of the possible [MangaField] values
pub fn all_common_fields() -> MangaCommonFields {
    let mut vec = Vec::with_capacity(MangaField::iter().len());
    for variant in MangaField::iter() {
        vec.push(variant);
    }
    MangaCommonFields(vec)
}

/// Return all of the possible [MangaDetail] fields
pub fn all_detail_fields() -> MangaDetailFields {
    let mut vec = Vec::with_capacity(MangaDetail::iter().len());
    for variant in MangaDetail::iter() {
        vec.push(variant);
    }
    MangaDetailFields(vec)
}

//! Module for interacting with the `anime` and `user animelist` endpoints

use self::requests::{AnimeCommonFields, AnimeDetail, AnimeDetailFields, AnimeField};
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
pub fn all_common_fields() -> AnimeCommonFields {
    let mut vec = Vec::with_capacity(AnimeField::iter().len());
    for variant in AnimeField::iter() {
        vec.push(variant);
    }
    AnimeCommonFields(vec)
}

/// Return all of the possible Anime Fields
pub fn all_detail_fields() -> AnimeDetailFields {
    let mut vec = Vec::with_capacity(AnimeDetail::iter().len());
    for variant in AnimeDetail::iter() {
        vec.push(variant);
    }
    AnimeDetailFields(vec)
}

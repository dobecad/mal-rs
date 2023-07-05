//! Module for interacting with the `user` endpoint

use self::{requests::UserFields, responses::UserEnum};
use strum::IntoEnumIterator;

/// User API client
pub mod api;

/// User API errors
pub mod error;

/// User API request structs
pub mod requests;

/// User API responses
pub mod responses;

/// Return all of the possible User Fields
pub fn all_fields() -> UserFields {
    let mut vec = Vec::with_capacity(UserEnum::iter().len());
    for variant in UserEnum::iter() {
        vec.push(variant);
    }
    UserFields(vec)
}

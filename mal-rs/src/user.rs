//! Module for interacting with the `user` endpoint

use self::{requests::UserFields, responses::UserFieldsEnum};
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
    let mut vec = Vec::with_capacity(UserFieldsEnum::iter().len());
    for variant in UserFieldsEnum::iter() {
        vec.push(variant);
    }
    UserFields(vec)
}

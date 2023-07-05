//! Module for interacting with the `user` endpoint

use self::{requests::UserFields, responses::UserEnum};
use strum::IntoEnumIterator;

/// API client
pub mod api;

/// API errors
pub mod error;

/// API requests
pub mod requests;

/// API responses
pub mod responses;

/// Return all of the possible User Fields
pub fn all_fields() -> UserFields {
    let mut vec = Vec::with_capacity(UserEnum::iter().len());
    for variant in UserEnum::iter() {
        vec.push(variant);
    }
    UserFields(vec)
}

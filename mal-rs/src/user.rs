use self::responses::UserEnum;
use strum::IntoEnumIterator;
use self::requests::UserFields;

pub mod api;
pub mod requests;
pub mod responses;
pub mod error;

pub fn all_fields() -> UserFields {
    let mut vec = Vec::with_capacity(UserEnum::iter().len());
    for variant in UserEnum::iter() {
        vec.push(variant);
    }
    UserFields(vec)
}
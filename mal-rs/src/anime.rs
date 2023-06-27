use crate::anime::responses::AnimeFieldsEnum;
use strum::IntoEnumIterator;

use self::requests::AnimeFields;

pub mod api;
pub mod error;
pub mod requests;
pub mod responses;

pub fn all_fields() -> AnimeFields {
    let mut vec = Vec::with_capacity(AnimeFieldsEnum::iter().len());
    for variant in AnimeFieldsEnum::iter() {
        vec.push(variant);
    }
    AnimeFields(vec)
}

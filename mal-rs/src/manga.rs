use self::{requests::MangaFields, responses::MangaFieldsEnum};
use strum::IntoEnumIterator;

pub mod api;
pub mod error;
pub mod requests;
pub mod responses;

pub fn all_fields() -> MangaFields {
    let mut vec = Vec::with_capacity(MangaFieldsEnum::iter().len());
    for variant in MangaFieldsEnum::iter() {
        vec.push(variant);
    }
    MangaFields(vec)
}

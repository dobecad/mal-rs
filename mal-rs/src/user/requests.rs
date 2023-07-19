// Structs for crafting User Endpoint requests
use serde::Serialize;

use strum_macros::EnumIter;

#[derive(Debug, Serialize)]
pub struct GetUserInformation {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetUserInformation {
    /// Create new `Get user information` query
    pub fn new(fields: Option<&UserFields>) -> Self {
        Self {
            fields: fields.map(|f| f.into()),
        }
    }
}

#[derive(Debug, EnumIter, PartialEq)]
#[allow(non_camel_case_types)]
pub enum UserField {
    id,
    name,
    picture,
    gender,
    birthday,
    location,
    joined_at,
    anime_statistics,
    time_zone,
    is_supporter,
}

pub struct UserFields(pub Vec<UserField>);

impl Into<String> for &UserFields {
    fn into(self) -> String {
        let result = self
            .0
            .iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<String>>()
            .join(",");
        result
    }
}

// Structs for crafting User Endpoint requests
use serde::Serialize;

use super::responses::UserEnum;

#[derive(Debug, Serialize)]
pub struct GetUserInformation {
    fields: String,
}

impl GetUserInformation {
    pub fn new(fields: UserFields) -> Self {
        Self {
            fields: fields.into()
        }
    }
}


pub struct UserFields(pub Vec<UserEnum>);

impl Into<String> for UserFields {
    fn into(self) -> String {
        let result = self
            .0
            .into_iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<String>>()
            .join(",");
        result
    }
}
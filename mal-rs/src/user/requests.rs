// Structs for crafting User Endpoint requests
use serde::Serialize;

use super::responses::UserEnum;

#[derive(Debug, Serialize)]
pub struct GetUserInformation {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<String>,
}

impl GetUserInformation {
    pub fn new(fields: Option<&UserFields>) -> Self {
        Self {
            fields: fields.map(|f| f.into()),
        }
    }
}

pub struct UserFields(pub Vec<UserEnum>);

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

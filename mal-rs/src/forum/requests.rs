use serde::Serialize;

use super::error::ForumApiError;

#[derive(Debug, Serialize)]
pub struct GetForumTopicDetail {
    #[serde(skip_serializing)]
    pub(crate) topic_id: u32,
    limit: u8,
    offset: u32,
}

impl GetForumTopicDetail {
    pub fn new(topic_id: u32, limit: u8, offset: u32) -> Result<Self, ForumApiError> {
        if limit < 1 || limit > 100 {
            return Err(ForumApiError::new(
                "Limit must be between 1 and 100 inclusive".to_string(),
            ));
        }

        Ok(Self {
            topic_id,
            limit,
            offset,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GetForumTopics {
    board_id: u32,
    subboard_id: u32,
    limit: u8,
    offset: u32,
    q: String,
    topic_user_name: String,
    user_name: String,
    // TODO: Support additional sorting methods once MAL add them
    sort: String,
}

impl GetForumTopics {
    pub fn new(
        board_id: u32,
        subboard_id: u32,
        limit: u8,
        offset: u32,
        q: String,
        topic_user_name: String,
        user_name: String,
    ) -> Result<Self, ForumApiError> {
        if limit < 1 || limit > 100 {
            return Err(ForumApiError::new(
                "Limit must be between 1 and 100 inclusive".to_string(),
            ));
        }

        Ok(Self {
            board_id,
            subboard_id,
            limit,
            offset,
            q,
            topic_user_name,
            user_name,
            sort: "recent".to_string(),
        })
    }
}

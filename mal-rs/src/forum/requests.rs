use serde::Serialize;

use crate::common::limit_check;

use super::error::ForumApiError;

#[derive(Debug, Serialize)]
pub struct GetForumTopicDetail {
    #[serde(skip_serializing)]
    pub(crate) topic_id: u32,
    limit: u16,
    offset: u32,
}

impl GetForumTopicDetail {
    pub fn new(
        topic_id: u32,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, ForumApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            ForumApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        Ok(Self {
            topic_id,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GetForumTopics {
    board_id: u32,
    subboard_id: u32,
    limit: u16,
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
        limit: Option<u16>,
        offset: Option<u32>,
        q: String,
        topic_user_name: String,
        user_name: String,
    ) -> Result<Self, ForumApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            ForumApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        Ok(Self {
            board_id,
            subboard_id,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
            q,
            topic_user_name,
            user_name,
            sort: "recent".to_string(),
        })
    }
}

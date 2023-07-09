use serde::Serialize;

use crate::common::limit_check;

use super::error::ForumApiError;

/// Corresponds to the [Get forum topic detail](https://myanimelist.net/apiconfig/references/api/v2#operation/forum_topic_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetForumTopicDetail {
    #[serde(skip_serializing)]
    pub(crate) topic_id: u32,
    limit: u16,
    offset: u32,
}

impl GetForumTopicDetail {
    /// Create new `Get forum topic detail` query
    /// 
    /// Limit must be within `[1, 100]`
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

/// Corresponds to the [Get forum topics](https://myanimelist.net/apiconfig/references/api/v2#operation/forum_topics_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetForumTopics {
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    board_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subboard_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<String>,
    limit: u16,
    offset: u32,
    // TODO: Support additional sorting methods once MAL add them
    sort: String,
}

impl GetForumTopics {
    /// Create new `Get forum topics` query
    /// 
    /// Limit must be within `[1, 100]`
    pub fn new(
        q: Option<String>,
        board_id: Option<u32>,
        subboard_id: Option<u32>,
        topic_user_name: Option<String>,
        user_name: Option<String>,
        limit: Option<u16>,
        offset: Option<u32>,
    ) -> Result<Self, ForumApiError> {
        limit_check(limit, 1, 100).map_err(|_| {
            ForumApiError::new("Limit must be between 1 and 100 inclusive".to_string())
        })?;

        if !(q.is_some()
            || board_id.is_some()
            || subboard_id.is_some()
            || topic_user_name.is_some()
            || user_name.is_some())
        {
            return Err(ForumApiError::new(
                "At least one of the optional arguments must be Some, excluding limit and offset"
                    .to_string(),
            ));
        }

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

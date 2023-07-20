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

    /// Use builder pattern for building up the query with required arguments
    pub fn builder(topic_id: u32) -> GetForumTopicDetailBuilder {
        GetForumTopicDetailBuilder::new(topic_id)
    }
}

pub struct GetForumTopicDetailBuilder {
    topic_id: u32,
    limit: Option<u16>,
    offset: Option<u32>,
}

impl GetForumTopicDetailBuilder {
    pub fn new(topic_id: u32) -> Self {
        Self {
            topic_id,
            limit: None,
            offset: None,
        }
    }

    pub fn topic_id(mut self, value: u32) -> Self {
        self.topic_id = value;
        self
    }

    pub fn limit(mut self, value: u16) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn build(self) -> Result<GetForumTopicDetail, ForumApiError> {
        GetForumTopicDetail::new(self.topic_id, self.limit, self.offset)
    }
}

/// Corresponds to the [Get forum topics](https://myanimelist.net/apiconfig/references/api/v2#operation/forum_topics_get) endpoint
#[derive(Debug, Serialize)]
pub struct GetForumTopics {
    nsfw: bool,
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
        nsfw: bool,
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
            nsfw,
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

    /// Use builder pattern for building up the query with required arguments
    pub fn builder() -> GetForumTopicsBuilder {
        GetForumTopicsBuilder::new()
    }
}

pub struct GetForumTopicsBuilder {
    nsfw: bool,
    q: Option<String>,
    board_id: Option<u32>,
    subboard_id: Option<u32>,
    topic_user_name: Option<String>,
    user_name: Option<String>,
    limit: Option<u16>,
    offset: Option<u32>,
}

impl GetForumTopicsBuilder {
    pub fn new() -> Self {
        Self {
            nsfw: false,
            q: None,
            board_id: None,
            subboard_id: None,
            topic_user_name: None,
            user_name: None,
            limit: None,
            offset: None,
        }
    }

    pub fn enable_nsfw(mut self) -> Self {
        self.nsfw = true;
        self
    }

    pub fn q(mut self, value: &str) -> Self {
        self.q = Some(value.to_string());
        self
    }

    pub fn board_id(mut self, value: u32) -> Self {
        self.board_id = Some(value);
        self
    }

    pub fn subboard_id(mut self, value: u32) -> Self {
        self.subboard_id = Some(value);
        self
    }

    pub fn topic_user_name(mut self, value: &str) -> Self {
        self.topic_user_name = Some(value.to_string());
        self
    }

    pub fn user_name(mut self, value: &str) -> Self {
        self.user_name = Some(value.to_string());
        self
    }

    pub fn limit(mut self, value: u16) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn build(self) -> Result<GetForumTopics, ForumApiError> {
        GetForumTopics::new(
            self.nsfw,
            self.q,
            self.board_id,
            self.subboard_id,
            self.topic_user_name,
            self.user_name,
            self.limit,
            self.offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_forum_topic_detail() {
        let query = GetForumTopicDetail::new(1234, Some(101), None);
        assert!(query.is_err());

        let query = GetForumTopicDetail::new(1234, Some(0), None);
        assert!(query.is_err());

        let query = GetForumTopicDetail::new(1234, Some(1), None);
        assert!(query.is_ok());

        let query = GetForumTopicDetail::new(1234, None, None);
        assert!(query.is_ok());
    }

    #[test]
    fn test_get_forum_topics() {
        let query = GetForumTopics::new(false, None, None, None, None, None, None, None);
        assert!(query.is_err());

        let query = GetForumTopics::new(
            false,
            Some("hello".to_string()),
            None,
            None,
            None,
            None,
            Some(101),
            None,
        );
        assert!(query.is_err());

        let query = GetForumTopics::new(
            false,
            Some("hello".to_string()),
            None,
            None,
            None,
            None,
            Some(0),
            None,
        );
        assert!(query.is_err());

        let query = GetForumTopics::new(
            false,
            Some("hello".to_string()),
            None,
            None,
            None,
            None,
            Some(100),
            None,
        );
        assert!(query.is_ok());

        let query = GetForumTopics::new(
            false,
            Some("hello".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(query.is_ok());
    }
}

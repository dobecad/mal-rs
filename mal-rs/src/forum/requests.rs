use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetForumTopicDetail {
    #[serde(skip_serializing)]
    pub(crate) topic_id: u32,
    limit: u8,
    offset: u32,
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
    // sort: String
}
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::common::{Paging, PagingIter};

#[derive(Debug, Deserialize, Serialize)]
pub struct ForumBoards {
    pub categories: Vec<Category>,
}

impl Display for ForumBoards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub title: String,
    pub boards: Vec<Board>,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Board {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub subboards: Vec<Subboard>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subboard {
    pub id: u32,
    pub title: String,
}

impl Display for Subboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForumTopicDetail {
    pub data: Vec<TopicDetail>,
    pub paging: Paging,
}

impl Display for ForumTopicDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

impl PagingIter for ForumTopicDetail {
    type Item = Self;

    fn next_page(&self) -> &Option<String> {
        &self.paging.next
    }

    fn prev_page(&self) -> &Option<String> {
        &self.paging.previous
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicDetail {
    pub title: String,
    pub posts: Vec<Post>,
    pub poll: Poll,
}

impl Display for TopicDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: u32,
    pub number: u32,
    pub created_at: String,
    pub created_by: ForumTopicPostCreatedBy,
    pub body: String,
    pub signature: String,
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForumTopicPostCreatedBy {
    pub id: u32,
    pub name: String,
    pub forum_avator: String,
}

impl Display for ForumTopicPostCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Poll {
    pub id: u32,
    pub question: String,
    pub close: bool,
    pub options: PollOptions,
}

impl Display for Poll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PollOptions {
    pub id: u32,
    pub text: String,
    pub votes: u32,
}

impl Display for PollOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForumTopics {
    pub data: Vec<ForumTopic>,
    pub paging: Paging,
}

impl Display for ForumTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

impl PagingIter for ForumTopics {
    type Item = Self;

    fn next_page(&self) -> &Option<String> {
        &self.paging.next
    }

    fn prev_page(&self) -> &Option<String> {
        &self.paging.previous
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForumTopic {
    pub id: u32,
    pub title: String,
    pub created_at: String,
    pub created_by: ForumTopicUser,
    pub number_of_posts: u32,
    pub last_post_created_at: String,
    pub last_post_created_by: ForumTopicUser,
    pub is_locked: bool,
}

impl Display for ForumTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForumTopicUser {
    pub id: u32,
    pub name: String,
}

impl Display for ForumTopicUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

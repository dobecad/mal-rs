use serde::Deserialize;

use crate::common::{Paging, PagingIter};

#[derive(Debug, Deserialize)]
pub struct ForumBoards {
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub title: String,
    pub boards: Vec<Board>,
}

#[derive(Debug, Deserialize)]
pub struct Board {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub subboards: Vec<Subboard>,
}

#[derive(Debug, Deserialize)]
pub struct Subboard {
    pub id: u32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopicDetail {
    pub data: Vec<TopicDetail>,
    pub paging: Paging,
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

#[derive(Debug, Deserialize)]
pub struct TopicDetail {
    pub title: String,
    pub posts: Vec<Post>,
    pub poll: Poll,
}

#[derive(Debug, Deserialize)]
pub struct Post {
    pub id: u32,
    pub number: u32,
    pub created_at: String,
    pub created_by: ForumTopicPostCreatedBy,
    pub body: String,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopicPostCreatedBy {
    pub id: u32,
    pub name: String,
    pub forum_avator: String,
}

#[derive(Debug, Deserialize)]
pub struct Poll {
    pub id: u32,
    pub question: String,
    pub close: bool,
    pub options: PollOptions,
}

#[derive(Debug, Deserialize)]
pub struct PollOptions {
    pub id: u32,
    pub text: String,
    pub votes: u32,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopics {
    pub data: Vec<ForumTopic>,
    pub paging: Paging,
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ForumTopicUser {
    pub id: u32,
    pub name: String,
}

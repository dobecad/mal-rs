use serde::Deserialize;

use crate::common::Paging;

#[derive(Debug, Deserialize)]
pub struct ForumBoards {
    categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    title: String,
    boards: Vec<Board>,
}

#[derive(Debug, Deserialize)]
pub struct Board {
    id: u32,
    title: String,
    description: String,
    subboards: Vec<Subboard>,
}

#[derive(Debug, Deserialize)]
pub struct Subboard {
    id: u32,
    title: String,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopicDetail {
    data: Vec<TopicDetail>,
    paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct TopicDetail {
    title: String,
    posts: Vec<Post>,
    poll: Poll,
}

#[derive(Debug, Deserialize)]
pub struct Post {
    id: u32,
    number: u32,
    created_at: String,
    created_by: ForumTopicPostCreatedBy,
    body: String,
    signature: String,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopicPostCreatedBy {
    id: u32,
    name: String,
    forum_avator: String,
}

#[derive(Debug, Deserialize)]
pub struct Poll {
    id: u32,
    question: String,
    close: bool,
    options: PollOptions,
}

#[derive(Debug, Deserialize)]
pub struct PollOptions {
    id: u32,
    text: String,
    votes: u32,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopics {
    data: Vec<ForumTopic>,
    paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopic {
    id: u32,
    title: String,
    created_at: String,
    created_by: ForumTopicUser,
    number_of_posts: u32,
    last_post_created_at: String,
    last_post_created_by: ForumTopicUser,
    is_locked: bool,
}

#[derive(Debug, Deserialize)]
pub struct ForumTopicUser {
    id: u32,
    name: String,
}

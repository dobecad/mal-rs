use dotenvy;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();
    let api_client = ForumApiClient::from(&client_id);
    let nsfw = false;
    let limit = Some(3);

    let topics = api_client.get_forum_boards().await;
    if let Ok(topics) = topics {
        println!("Topics: {}\n", topics);
    }

    let query = GetForumTopicDetail::new(481, limit, None).unwrap();
    let response = api_client.get_forum_topic_detail(&query).await;
    if let Ok(response) = response {
        println!("Forum topic detail: {}\n", response);
    }

    let query = GetForumTopics::new(
        nsfw,
        Some("hello".to_string()),
        None,
        None,
        None,
        None,
        limit,
        None,
    )
    .unwrap();
    let response = api_client.get_forum_topics(&query).await;
    if let Ok(response) = response {
        println!("Forum topics: {}", response)
    }
}

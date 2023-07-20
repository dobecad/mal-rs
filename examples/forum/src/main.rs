use dotenvy;
use mal_api::oauth::MalClientId;
use mal_api::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();
    let api_client = ForumApiClient::from(&client_id);
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

    let query = GetForumTopics::builder()
        .q("hello")
        .enable_nsfw()
        .limit(5)
        .build()
        .unwrap();
    let response = api_client.get_forum_topics(&query).await;
    if let Ok(response) = response {
        println!("Forum topics: {}", response)
    }
}

use std::error::Error;

use futures::{stream, StreamExt};
use reqwest::Client;
use serde_json::{json, Map, Value};
use tokio::time::{Duration, timeout};
use uuid::Uuid;

use crate::structs::{Article, ImportResult};

pub async fn save_urls(key: String, articles: &Vec<Article>) -> Vec<ImportResult> {
    let client = Client::new();

    stream::iter(articles)
        .then(|article| {
            let key = key.clone();
            let client = client.clone();
            process_article(client, key, article)
        })
        .collect()
        .await
}

async fn process_article(client: Client, key: String, article: &Article) -> ImportResult {
    let article_url = article.url.to_string();
    match check_valid_url(&client, &article_url).await {
        Ok(is_valid_url) => {
            if is_valid_url {
                save_url(&client, &key, article).await
            } else {
                create_import_result(article_url, false, true, None)
            }
        }
        Err(error) => {
            let error_message = format!("URL could not be validated {}: {}", article_url, error);
            eprintln!("{}", error_message);
            create_import_result(article_url, false, false, Some(error_message.to_string()))
        }
    }
}

async fn check_valid_url(client: &Client, article_url: &str) -> Result<bool, Box<dyn Error>> {
    let response = client.get(article_url).send().await?;
    Ok(response.status().is_success())
}

async fn save_url(client: &Client, key: &str, article: &Article) -> ImportResult {
    let payload = json!({
        "query": "mutation SaveUrl($input: SaveUrlInput!) { \
            saveUrl(input: $input) { \
                ... on SaveSuccess { url clientRequestId } \
                ... on SaveError { errorCodes message } \
                } \
            }",
        "variables": {
            "input": create_input(article)
        }
    });

    let timeout_duration = Duration::from_secs(30);
    let result = timeout(
        timeout_duration,
        client.post("https://api-prod.omnivore.app/api/graphql")
            .json(&payload)
            .header("content-type", "application/json")
            .header("authorization", key)
            .send(),
    ).await;

    let article_url = article.url.to_string();
    match result {
        Ok(Ok(response)) => {
            if response.status().is_success() {
                // TODO remove these two lines at the end
                let result_body = response.text().await;
                println!("Resulting body {:#?}", result_body);
                create_import_result(article_url, true, false, None)
            } else {
                let status = response.status();
                let text = response.text().await;
                let error_message = format!("Error occurred during the saving of the URL {}. Server returned \"{}\" and the message {}", article_url, status, text.unwrap());
                create_import_result(article_url, false, false, Some(error_message))
            }
        }
        Ok(Err(error)) => {
            let error_message = format!("Error occurred during the saving of the URL {}: {}", article_url, error);
            create_import_result(article.url.to_string(), false, false, Some(error_message))
        }
        Err(error) => {
            let error_message = format!("Timeout occurred for the saving of the URL {}: {}", article_url, error);
            create_import_result(article.url.to_string(), false, false, Some(error_message))
        }
    }
}

fn create_input(article: &Article) -> Map<String, Value> {
    let article_url = article.url.to_string();
    let saved_date = article.saved_date.to_string();
    let location = article.location.to_string();
    let is_archived = location == "archive";

    let mut input_map = serde_json::Map::new();
    input_map.insert("clientRequestId".to_string(), Value::String(format!("{}", Uuid::new_v4())));
    input_map.insert("source".to_string(), Value::String("api".to_string()));
    input_map.insert("url".to_string(), Value::String(format!("{}", article_url)));
    // TODO place this back
    // input_map.insert("savedAt".to_string(), Value::String(format!("{}", saved_date)));
    input_map.insert("labels".to_string(), json!([{"name": "imported"}]));
    if is_archived {
        input_map.insert("state".to_string(), Value::String("ARCHIVED".to_string()));
    }

    input_map
}

fn create_import_result(url: String, successful: bool, is_invalid_url: bool, error: Option<String>) -> ImportResult {
    ImportResult {
        url,
        successful,
        is_invalid_url,
        error,
    }
}
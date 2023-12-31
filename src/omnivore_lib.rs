use std::error::Error;
use std::sync::Arc;

use futures::{stream, StreamExt};
use reqwest::Client;
use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::structs::{Article, ImportedArticle};

pub async fn save_urls(key: String, imported_articles: &Vec<Article>) -> Vec<ImportedArticle> {
    let atomic_key = Arc::new(key);
    let client = Client::new();

    stream::iter(imported_articles)
        .then(|article| {
            let key = Arc::clone(&atomic_key).to_string();
            let client = client.clone();
            async move {
                let article_url = article.url.to_string();
                let saved_date = article.saved_date.to_string();
                let location = article.location.to_string();
                let is_archived = location == "archive";
                let input = create_input(&article_url, &saved_date, is_archived);

                match check_valid_url(&client, &article_url).await {
                    Ok(is_valid_url) => {
                        if is_valid_url {
                            match save_url(input, key, &client).await {
                                Ok(_) => ImportedArticle { url: article_url, successful: true, is_invalid_url: false, error: None },
                                Err(error) => {
                                    let error_message = format!("Error has occurred during the saving of URLs into Omnivore:{}", error);
                                    ImportedArticle { url: article_url, successful: false, is_invalid_url: false, error: Some(error_message.to_string()) }
                                }
                            }
                        } else {
                            ImportedArticle { url: article_url, successful: false, is_invalid_url: true, error: None }
                        }
                    }
                    Err(error) => {
                        let error_message = format!("URL could not be validated: {}", error);
                        eprintln!("{}", error_message);
                        ImportedArticle { url: article_url, successful: false, is_invalid_url: false, error: Some(error_message.to_string()) }
                    }
                }
            }
        })
        .collect()
        .await
}

fn create_input(article_url: &str, saved_date: &str, is_archived: bool) -> Map<String, Value> {
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

async fn check_valid_url(client: &Client, article_url: &str) -> Result<bool, Box<dyn Error>> {
    let response = client.get(article_url).send().await?;
    Ok(response.status().is_success())
}

async fn save_url(input: Map<String, Value>, key: String, client: &Client) -> Result<(), Box<dyn Error>> {
    let payload = json!({
        "query": "mutation SaveUrl($input: SaveUrlInput!) { \
            saveUrl(input: $input) { \
                ... on SaveSuccess { url clientRequestId } \
                ... on SaveError { errorCodes message } \
                } \
            }",
        "variables": {
            "input": input
        }
    });

    let result = client.post("https://api-prod.omnivore.app/api/graphql")
        .json(&payload)
        .header("content-type", "application/json")
        .header("authorization", key)
        .send()
        .await;

    match result {
        Ok(response) => {
            if response.status().is_success() {
                // TODO remove these two lines at the end
                let result_body = response.text().await?;
                println!("Resulting body {:#?}", result_body);
                Ok(())
            } else {
                let status = response.status();
                let text = response.text().await?;
                let error_message = format!("Server returned the code \"{}\" and the message {}", status, text);
                Err(error_message.into())
            }
        }
        Err(error) => {
            let error_message = format!("Error while processing request: {}", error);
            Err(error_message.into())
        }
    }
}



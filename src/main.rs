use std::{error::Error, process};
use itertools::Either::Left;
use itertools::Either::Right;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
struct Article {
    #[serde(rename(deserialize = "Title"))]
    title: String,
    #[serde(rename(deserialize = "URL"))]
    url: String,
    #[serde(rename(deserialize = "Document tags"))]
    document_tags: String,
    #[serde(rename(deserialize = "Saved date"))]
    saved_date: String,
    #[serde(rename(deserialize = "Reading progress"))]
    reading_progress: String,
    #[serde(rename(deserialize = "Location"))]
    location: String,
    #[serde(rename(deserialize = "Seen"))]
    seen: String,
}

fn get_imported_articles() -> Result<(Vec<Article>), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path("test.csv")?;
    let (errors, articles): (Vec<csv::Error>, Vec<Article>) = csv_reader
        .deserialize()
        .partition_map(|row| match row {
            Err(e) => Left(e),
            Ok(article) => Right(article)
        });

    if errors.is_empty() {
        Ok(articles)
    } else {
        Err("Errors occurred while reading the CSV".into())
    }
}

async fn save_url(article_url: String, saved_date: String, is_archived: bool) -> Result<(), Box<dyn Error>> {
    let mut input_map = serde_json::Map::new();

    input_map.insert("clientRequestId".to_string(), Value::String(format!("{}", Uuid::new_v4())));
    input_map.insert("source".to_string(), Value::String("api".to_string()));
    input_map.insert("url".to_string(), Value::String(format!("{}", article_url)));
    input_map.insert("savedAt".to_string(), Value::String(format!("{}", saved_date)));
    input_map.insert("labels".to_string(), json!([{"name": "imported"}]));
    if is_archived {
        input_map.insert("state".to_string(), Value::String("ARCHIVED".to_string()));
    }

    let payload = json!({
        "query": "mutation SaveUrl($input: SaveUrlInput!) { saveUrl(input: $input) { ... on SaveSuccess { url clientRequestId } ... on SaveError { errorCodes message } } }",
        "variables": {
            "input": input_map
        }
    });

    // println!("Payload");
    // println!("{}", payload.to_string());

    let client = reqwest::Client::new();
    let result = client.post("https://api-prod.omnivore.app/api/graphql")
        .json(&payload)
        .header("content-type", "application/json")
        .header("authorization", "MY API KEY SHOULD BE HERE")
        .send()
        .await?;
    let result_body = result.text().await?;
    println!("Resulting body {:#?}", result_body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let imported_articles = get_imported_articles()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });

    // println!("Doing this from the main");
    // imported_articles.iter().for_each(|article: &Article| println!("{:#?}", article));

    let article_url = imported_articles.get(0).unwrap().url.to_string();
    let saved_date = imported_articles.get(0).unwrap().saved_date.to_string();
    save_url(article_url, saved_date, false).await
        .unwrap_or_else(|error| {
            eprintln!("error occurred")
        });

    println!("Successfully imported csv into Omnivore");
    Ok(())
}

use std::{error::Error, io, process};
use itertools::Either::Left;
use itertools::Either::Right;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::json;
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

    // println!("{:#?}", articles);
    // println!("{:#?}", errors);

    if errors.is_empty() {
        Ok(articles)
    } else {
        Err("Errors occurred while reading the CSV".into())
    }
}
// curl -X POST -d '{ "query": "mutation SaveUrl($input: SaveUrlInput!) { saveUrl(input: $input) { ... on SaveSuccess { url clientRequestId } ... on SaveError { errorCodes message } } }", "variables": { "input": { "clientRequestId": "85282635-4DF4-4BFC-A3D4-B3A004E57067", "source": "api", "url": "https://blog.omnivore.app/p/contributing-to-omnivore" }} }' -H 'content-type: application/json' -H 'authorization: <your api key>' https://api-prod.omnivore.app/api/graphql


async fn save_article(article_url: String) -> Result<(), Box<dyn Error>> {
    let payload = json!({
        "query": "mutation SaveUrl($input: SaveUrlInput!) { saveUrl(input: $input) { ... on SaveSuccess { url clientRequestId } ... on SaveError { errorCodes message } } }",
        "variables": {
            "input": {
                "clientRequestId": format!("{}", Uuid::new_v4()),
                "source": "api",
                "url": format!("{}", article_url)
            }
        }
    });
    // println!("Payload");
    // println!("{}", payload.to_string());
    let client = reqwest::Client::new();
    let res = client.post("https://api-prod.omnivore.app/api/graphql")
        .json(&payload)
        .header("content-type", "application/json")
        .header("authorization", "MY API KEY SHOULD BE HERE")
        .send()
        .await?;

    // let body = reqwest::get("https://www.rust-lang.org").await?
    //     .text().await?;
    println!("Let's print the body");
    // println!("{:#?}", res);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let imported_articles = get_imported_articles()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });

    println!("Doing this from the main");
    imported_articles.iter().for_each(|article: &Article| println!("{:#?}", article));

    let article_url = imported_articles.get(0).unwrap().url.to_string();
    save_article(article_url).await
        .unwrap_or_else(|error| {
            eprintln!("error occurred")
        });

    Ok(())
}

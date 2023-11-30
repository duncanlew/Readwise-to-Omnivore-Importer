use std::{error::Error, io, process};
use itertools::Either::Left;
use itertools::Either::Right;
use itertools::Itertools;
use serde::Deserialize;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let imported_articles = get_imported_articles()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });

    println!("Doing this from the main");
    imported_articles.iter().for_each(|article: &Article| println!("{:#?}", article));

    let body = reqwest::get("https://www.rust-lang.org").await?
        .text().await?;
    println!("Let's print the body");
    println!("{:#?}", body);

    Ok(())
}

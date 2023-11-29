use std::{error::Error, io, process};
use itertools::Either::Left;
use itertools::Either::Right;
use itertools::Itertools;

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
    let (errors, articles): (Vec<_>, Vec<Article>) = csv_reader
        .deserialize()
        .partition_map(|row| match row {
            Err(e) => Left(e),
            Ok(article) => Right(article)
        });

    println!("{:#?}", articles);
    println!("{:#?}", errors);

    if errors.is_empty() {
        Ok(articles)
    } else {
        Err("Errors occurred while reading the CSV".into())
    }
}

fn main() {
    if let Err(err) = get_imported_articles() {
        println!("Error running example: {}", err);
        process::exit(1);
    }
}

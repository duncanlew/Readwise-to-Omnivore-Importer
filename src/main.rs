use std::{error::Error, io, process};

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
    seen: String
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path("test.csv")?;

    let articles: Vec<Article> = csv_reader
        .deserialize()
        .filter_map(|row| row.ok())
        .collect();
    println!("{:#?}", articles);


    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("Error running example: {}", err);
        process::exit(1);
    }
}

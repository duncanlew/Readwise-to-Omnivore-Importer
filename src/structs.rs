use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Duncan Lew", version, about)]
/// A Readwise to Omnivore importer
pub struct Arguments {
    #[clap(short, long)]
    /// API key for Omnivore
    pub(crate) key: String,

    #[clap(short, long)]
    /// File path for the CSV file
    pub(crate) file_path: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Article {
    #[serde(rename(deserialize = "Title"))]
    pub(crate) title: String,
    #[serde(rename(deserialize = "URL"))]
    pub(crate) url: String,
    #[serde(rename(deserialize = "Document tags"))]
    pub(crate) document_tags: String,
    #[serde(rename(deserialize = "Saved date"))]
    pub(crate) saved_date: String,
    #[serde(rename(deserialize = "Reading progress"))]
    pub(crate) reading_progress: String,
    #[serde(rename(deserialize = "Location"))]
    pub(crate) location: String,
    #[serde(rename(deserialize = "Seen"))]
    pub(crate) seen: String,
}
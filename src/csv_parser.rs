use std::error::Error;

use itertools::Either::{Left, Right};
use itertools::Itertools;

use crate::structs::{Article, ImportedArticle};

pub fn get_imported_articles(file_path: &str) -> Result<Vec<Article>, Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path(file_path)?;
    let (errors, articles): (Vec<csv::Error>, Vec<Article>) = csv_reader
        .deserialize()
        .partition_map(|row| match row {
            Err(e) => Left(e),
            Ok(article) => Right(article)
        });

    if errors.is_empty() {
        Ok(articles)
    } else {
        eprintln!("For the file {} the following CSV parsing errors occurred:\n{:#?}", file_path, errors);
        Err("Errors occurred while reading the CSV".into())
    }
}

pub fn write_logs(articles: Vec<Article>, invalid_results: Vec<ImportedArticle>, error_results: Vec<ImportedArticle>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("foo.csv")?;

    let invalid_urls: std::collections::HashSet<&str> = invalid_results
        .iter()
        .map(|imported_article| imported_article.url.as_str())
        .collect();

     articles
        .iter()
        .filter(|article| invalid_urls.contains(article.url.as_str()))
        .try_for_each(|article| wtr.serialize(article))?;

    wtr.flush()?;
    Ok(())
}
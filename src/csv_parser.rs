use std::error::Error;
use std::process::exit;
use chrono::Local;

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

pub fn write_logs(articles: Vec<Article>, invalid_results: Vec<ImportedArticle>, error_results: Vec<ImportedArticle>) {
    let timestamp = Local::now().format("%Y-%m-%d--%H-%M-%S").to_string();

    let invalid_urls: std::collections::HashSet<&str> = invalid_results
        .iter()
        .map(|imported_article| imported_article.url.as_str())
        .collect();
    let error_urls: std::collections::HashSet<&str> = error_results
        .iter()
        .map(|imported_article| imported_article.url.as_str())
        .collect();

    write_logs_for_invalid_articles(&timestamp, invalid_urls, articles)
        .unwrap_or_else(|err| {
            eprintln!("Error occurred during the saving of the logs for invalid articles {}", err);
        });
    // write_logs_for_error_articles(&timestamp, error_urls, &articles)
    //     .unwrap_or_else(|err| {
    //         eprintln!("Error occurred during the saving of the logs for error articles {}", err);
    //     });
}

fn write_logs_for_invalid_articles(timestamp: &str, invalid_urls: std::collections::HashSet<&str>, articles: Vec<Article>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(format!("invalid-articles-{}.csv", timestamp))?;

    articles
        .iter()
        .filter(|article| invalid_urls.contains(article.url.as_str()))
        .try_for_each(|article| wtr.serialize(article))?;

    wtr.flush()?;
    Ok(())
}

fn write_logs_for_error_articles(timestamp: &str, error_urls: std::collections::HashSet<&str>, articles: &Vec<Article>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(format!("invalid-articles-{}.csv", timestamp))?;

    articles
        .iter()
        .filter(|article| error_urls.contains(article.url.as_str()))
        .try_for_each(|article| wtr.serialize(article))?;

    wtr.flush()?;
    Ok(())
}
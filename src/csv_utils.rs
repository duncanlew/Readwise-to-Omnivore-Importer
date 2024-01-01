use std::collections::HashSet;
use std::error::Error;

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
        let error_message = format!("For the file {} the following CSV parsing errors occurred:\n{:#?}", file_path, errors);
        Err(error_message.into())
    }
}

pub fn write_logs(articles: Vec<Article>, invalid_results: Vec<ImportedArticle>, error_results: Vec<ImportedArticle>) {
    let timestamp = Local::now().format("%Y-%m-%d--%H-%M-%S").to_string();

    write_logs_for_articles("invalid", &timestamp, &articles, &invalid_results)
        .unwrap_or_else(|err| {
            eprintln!("Error occurred during the saving of the logs for invalid articles: {}", err);
        });
    write_logs_for_articles("error", &timestamp, &articles, &error_results)
        .unwrap_or_else(|err| {
            eprintln!("Error occurred during the saving of the logs for error articles: {}", err);
        });
}

fn write_logs_for_articles(log_type: &str, timestamp: &str, articles: &Vec<Article>, results: &Vec<ImportedArticle>) -> Result<(), Box<dyn Error>> {
    let urls: HashSet<&str> = results
        .iter()
        .map(|imported_article| imported_article.url.as_str())
        .collect();

    if urls.is_empty() {
        return Ok(());
    }

    let file_name = format!("{}-articles-{}.csv", log_type, timestamp);
    let mut wtr = csv::Writer::from_path(file_name)?;

    articles
        .iter()
        .filter(|article| urls.contains(article.url.as_str()))
        .try_for_each(|article| wtr.serialize(article))?;

    wtr.flush()?;
    Ok(())
}
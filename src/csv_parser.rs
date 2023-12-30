use std::error::Error;

use itertools::Either::{Left, Right};
use itertools::Itertools;

use crate::structs::Article;

pub fn get_imported_articles(file_path: String) -> Result<Vec<Article>, Box<dyn Error>> {
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

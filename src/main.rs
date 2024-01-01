use std::error::Error;
use std::process::exit;

use clap::Parser;
use crate::csv_utils::write_logs;

use crate::structs::{Arguments, ImportedArticle};

mod structs;
mod csv_utils;
mod omnivore_lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();

    let articles = csv_utils::get_imported_articles(&arguments.file_path)
        .unwrap_or_else(|err| {
            eprintln!("Error occurred: {}\nExiting application", err);
            exit(1);
        });

    let results = omnivore_lib::save_urls(arguments.key, &articles).await;
    let (success_results, rest_results): (Vec<ImportedArticle>, Vec<ImportedArticle>) =  results.into_iter().partition(|result| result.successful);
    let (invalid_results, error_results): (Vec<ImportedArticle>, Vec<ImportedArticle>) = rest_results.into_iter().partition(|result| result.is_invalid_url);


    // TODO remove these lines
    println!("\n*************************\nInvalid results");
    println!("{:#?}", invalid_results);
    println!("*************************\nError results");
    println!("{:#?}", error_results);
    println!("*************************\nSuccess results");
    println!("{:#?}", success_results);
    // End of TODO

    let invalid_count = invalid_results.len();
    let error_count = error_results.len();
    let success_count = success_results.len();
    let total_count = invalid_count + error_count + success_count;
    println!("\n=============================================");
    println!("Total processed: {}", total_count);
    println!("\tInvalid count: {}", invalid_count);
    println!("\tError count: {}", error_count);
    println!("\tSuccess count: {}", success_count);

    write_logs(articles, invalid_results, error_results);
    Ok(())
}
mod structs;
mod csv_parser;
mod omnivore_lib;

use std::error::Error;
use std::process::exit;
use itertools::Itertools;
use serde::Deserialize;
use clap::Parser;
use futures::StreamExt;
use crate::structs::Arguments;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();

    let imported_articles = csv_parser::get_imported_articles(arguments.file_path)
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            exit(1);
        });

    omnivore_lib::save_urls(arguments.key, imported_articles).await;

    println!("Successfully imported csv into Omnivore");
    Ok(())
}

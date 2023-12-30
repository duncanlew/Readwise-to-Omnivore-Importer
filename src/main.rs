use std::error::Error;
use std::process::exit;

use clap::Parser;
use futures::StreamExt;
use itertools::Itertools;
use serde::Deserialize;

use crate::structs::Arguments;

mod structs;
mod csv_parser;
mod omnivore_lib;

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

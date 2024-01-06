# Readwise to Omnivore importer

The "Readwise to Omnivore Importer" is a custom tool built using Rust to import articles from Readwise Reader to
Omnivore. It uses a CSV file exported from Readwise Reader using
the [web interface](https://blog.readwise.io/p/f8c0f71c-fe5f-4025-af57-f9f65c53fed7/#howdoigenerateacsvofallmysaveddocuments).
The importer parses this CSV file and imports all the data into Omnivore. An extra check is built into the tool to
validate if a URL still exists before importing it into Omnivore. This is done to avoid polluting your Omnivore library
with broken links.
The tool also provides clear logging results and a summary of the import process.

## Features

- Parse the exported CSV file
- URLs that are invalid won't be imported into Omnivore
- Import into Omnivore using an API key for authentication
- Detailed logging of invalid and errored results shown in the terminal and stored as a CSV file
- Possibility to run the project locally or use the tool as a binary

## Prerequisites

- Exported CSV file from Readwise Reader using the web interface. Check
  the [FAQ](https://blog.readwise.io/p/f8c0f71c-fe5f-4025-af57-f9f65c53fed7/#howdoigenerateacsvofallmysaveddocuments)
  for the steps.
- [API key](https://docs.omnivore.app/integrations/api.html#getting-an-api-token) from Omnivore

## Running the importer tool

There are two ways to run the importer tool. The easiest way without requiring to install Rust is by using the
binary.
For those who would like to explore the repository or make changes to the source code, it's also possible to run the
tool with the Rust build tools.

### Running it with the binary

A compiled binary is available for download in the project GitHub Releases.

1. __Download the binary__

   Navigate to the [Release](https://github.com/duncanlew/readwise-to-omnivore-importer/releases) page and download the
   latest version for your operating system

2. __Run the binary__

   Open a terminal and navigate to the directory where you downloaded the binary. Add your CSV file in that directory
   and then run the following command with the two parameters replaced

   ```bash
   ./readwise_to_omnivore_importer --key YOUR_API_KEY --file-path PATH_TO_CSV
   ```

### Running it locally

To run the project locally, make sure to first install [Rust](https://www.rust-lang.org/tools/install) on your local
machine. This should only take a few minutes. When that is done, open up a terminal and follow these steps:

1. __Clone the repository__
    ```bash
   git clone git@github.com:duncanlew/readwise-to-omnivore-importer.git
   cd readwise-to-omnivore-importer
    ```
2. __Add your CSV file__

   Add your exported CSV file from Readwise Reader into the directory called `readwise-to-omnivore-importer`.
3. __Run the importer__
    ```bash
    cargo run -- --key YOUR_API_KEY --file-path PATH_TO_CSV
    ```

## Licence

This project is licensed under the MIT License. See the `LICENSE` file for more details. 
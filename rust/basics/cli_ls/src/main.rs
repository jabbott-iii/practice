use clap::{Parser, Subcommand};
use std::{env::current_dir, fs, path::PathBuf};
use owo_colors::OwoColorize;
use strum::Display;
use chrono::prelude::DateTime;
use tabled::{Tabled, settings::{Color, Style, object::Columns, object::Rows}};

// Command-line interface definition
#[derive(Debug, Parser)]
#[command(version, author = "Joseph Abbott III", about = "Omnia CLI - A versatile command-line interface")]
pub struct Cli {
    path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ls { // List directory contents command
        #[arg(value_name = "FILES")]
        path: Option<PathBuf>,
    },
    // Add other commands here (e.g., 'ls', 'mkdir')
}

fn main() {
    loop {
        // Display prompt and read user input
        println!("{}", "Enter the ls command (type 'exit' to quit):".bright_green());
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();

        if command.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting Omnia CLI. Goodbye!".bright_red());
            break;
        }
        if command.eq_ignore_ascii_case("ls") {
            ls_complete();
        }
    }
}

// Enum to represent file types
#[derive(Debug, Display)]
enum FileType {
    File,
    Directory,
}

// Structure to hold file information
#[derive(Debug, Tabled)]
struct FileInfo {
    #[tabled{rename = "Name"}]
    name: String,
    #[tabled{rename = "Type"}]
    path_type: FileType,
    #[tabled{rename = "Size (bytes)"}]
    size: u64,
    #[tabled{rename = "Modified"}]
    modified: String,
}

// Main function to execute the 'ls' command
fn ls_complete() {
    let args = Cli::parse();
    let path = args.path.unwrap_or(PathBuf::from(current_dir().unwrap()));
    println!("Current path: {}", path.display().bright_blue());
    if let Ok(does_exist) = fs::exists(&path)  {
        if does_exist {
            get_files_table(&path);
        }
    } else {
        println!("{}", "The path does not exist.".red());
    }
}

// Generate and display a table of files in the specified directory
fn get_files_table(path: &PathBuf) {
    let get_files = get_files(&path);
    let mut table = tabled::Table::new(get_files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_BLUE);
    table.modify(Columns::one(1), Color::FG_BRIGHT_BLUE);
    table.modify(Columns::one(2), Color::FG_BRIGHT_BLUE);
    table.modify(Columns::one(3), Color::FG_BRIGHT_BLUE);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table);
}

// Retrieve files from the specified directory path
fn get_files(path: &PathBuf) -> Vec<FileInfo> {
    let mut files = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                meta_data(&file, &mut files);
            }
        }
    } else {
        println!("{}", "Could not read the directory.".red());
    }
    files
}

// Extract metadata for a given file and append it to the files vector
fn meta_data(file: &fs::DirEntry, files: &mut Vec<FileInfo>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        files.push(
            FileInfo {
                name: file.file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                path_type: if meta.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                },
                size: meta.len(),
                modified: DateTime::<chrono::Utc>::from(
                    meta.modified()
                        .unwrap_or(std::time::SystemTime::now())
                )
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            }
        );
    }
}
//---------------------------------- END LS COMMANDS ----------------------------//

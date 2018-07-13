//! # minigrep
//!
//! `minigrep` is a tutorial project from the Rust programming book.

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/// Runs `minigrep`
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)
        .expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong while reading the file");

    match config.case_insensitive {
        true => {
            search_case_insensitive(&config.query, &contents)
                .iter()
                .for_each(|line| {
                    println!("{}", line);
                });
        },
        _ => {
            search(&config.query, &contents)
                .iter()
                .for_each(|line| {
                    println!("{}", line);
                });
        },
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        match args.len() >= 3 {
            true => {
                args.next();

                let query = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a query string"),
                };

                let filename = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a file name"),
                };

                let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

                Ok(Config {
                    query,
                    filename,
                    case_insensitive,
                })
            },
            _ => Err("not enough arguments"),
        }
    }
}

/// Searches for lines in a file matching an input string, case-sensitive
///
/// # Examples
///
/// ```
/// let query = "Hello";
/// let lines = "\
/// Hello
/// hello world
/// World";
///
/// assert_eq!(
///     vec!["Hello"],
///     minigrep::search(query, lines),
/// );
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.contains(query))
        .collect()
}

/// Searches for lines in a file matching an input string, case-insensitive
///
/// # Examples
///
/// ```
/// let query = "Hello";
/// let lines = "\
/// Hello
/// hello world
/// World";
///
/// assert_eq!(
///     vec!["Hello", "hello world"],
///     minigrep::search_case_insensitive(query, lines),
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

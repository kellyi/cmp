use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }
}

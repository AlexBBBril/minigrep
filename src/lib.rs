use clap::Parser;
use std::error::Error;
use std::fs;

/// Search configuration
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Config {

    /// Query string
    pub query: String,

    /// File path
    pub file_path: String,

    /// Ignore the register when searching
    #[arg(short = 'i', long)]
    pub ignore_case: bool,
}

impl Config {
    pub fn build(config: Config) -> Result<Config, &'static str> {
        let ignore_case = config.ignore_case;
        let query: String = config.query;
        let file_path: String = config.file_path;

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn parse_config() -> Config {
        Config::parse()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            lines.push(line);
        }
    }

    lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        let query = query.to_lowercase();
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        )
    }
}

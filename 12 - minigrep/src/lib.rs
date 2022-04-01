use std::{env, error::Error, fs};

/// The configuration struct
/// - [query] is query string you target
/// - [filename] is where file located
/// - [case_sensitive] ignore sensitive case flag
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &mut env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get a query string."),
        };
        let filename = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get a filename string."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Run command with config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let query = config.query;
    let filename = &config.filename;
    let contents = fs::read_to_string(&filename)?;
    let search_result = if config.case_sensitive {
        search_case_insensitive(&query, &contents)
    } else {
        search(&query, &contents)
    };

    for line in &search_result {
        println!("Search in: {line}")
    }

    if search_result.is_empty() {
        println!("Not found!");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|content| content.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&query, &contents),
        );
    }
}

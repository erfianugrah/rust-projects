//! # minigrep
//! `minigrep` is a tool to search strings within lines of text and printing out the line that
//! contains the "grep-ed" string
use std::env;
use std::error::Error;
use std::fs; // file system library

/// Trait object Box<dyn Error>, function
/// will return a type that implements the Error trait, but we don't have to specify, hence
///dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // ? will return the error from the
    let results = if config.ignore_case {
        // current function for the caller to handle

        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file"); // In main, the new statement fs::read_to_string takes the file_path, opens that file, and returns a std::io::Result<String> of the file’s contents.
    // println!("With text:\n{contents}");
    //
    Ok(()) // need to wrap () from Result in Ok()
           // dbg!(args);
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // error values will always be
        // string literls and have the 'static lifetime
        // fn new(args: &[String]) -> Config {
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        // let query = args[1].clone(); // can't take ownership of an iterator
        // let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // checking whether env var is set

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
    // let query = query.to_lowercase(); // so that it is case insensitive, but won't really work for
    //                                   // unicode
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         // .to_lowercase() creates a new String not a
    //         // string slice so & is needed since contains is defined to take a string slice
    //         results.push(line);
    //     }
    // }
    // results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     // line is an iterator
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    // // vec![]
}

// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1].clone(); // Instead of lifetimes, clone() makes the code a bit more
//                                   // simpler
//     let file_path = &args[2].clone();
//
//     Config { query, file_path }
// }

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
        );
    }
}

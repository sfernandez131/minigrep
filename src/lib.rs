use std::{env, error::Error, fmt, fs};

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args.
                useage: cargo run <query> <file_path> optional:<case_sensitive>");
        }
        let config = Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            case_sensitive: Self::get_case_sensitive(args),
        };
        //println!("{0}", config.case_sensitive);
        if env::var("IGNORE_CASE").is_ok() {
            println!(
                "Environment variable 'IGNORE_CASE' is set.
                Function will run case insensitive.
                If in PS type 'Remove-Item Env:IGNORE_CASE' to disable."
            );
        } else {
            println!(
                "IGNORE_CASE not set.
                If in PS type '$env:IGNORE_CASE=1; cargo <commands>' to set."
            );
        }
        Ok(config)
    }

    // Helps determine if the user has overridden any
    // environment vars
    fn get_case_sensitive(args: &[String]) -> bool {
        if args.len() == 4 {
            args[3] == "true"
        } else {
            env::var("IGNORE_CASE").is_ok()
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Searching for '{}' in {}", self.query, self.file_path)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    //println!("{config:#?}");

    for line in search(&config.query, &contents, &config.case_sensitive) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: &bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if *case_insensitive == false {
            if line.contains(query) {
                results.push(line.trim());
            }
        } else {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line.trim());
            }
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
        safe, fast, productive
        Pick three.
        Duct tape.";
        let case_sensitive = false;

        assert_eq!(
            vec!["safe, fast, productive"],
            search(query, contents, &case_sensitive)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive
        Pick three.
        Trust me.";
        let case_sensitive = true;

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, &case_sensitive)
        );
    }
}

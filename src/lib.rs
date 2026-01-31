use std::{error::Error, fs};

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args.");
        }
        let config = Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        };
        Ok(config)
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("Found text:\n'{contents}'");
    Ok(())
}

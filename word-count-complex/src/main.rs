use regex::Regex;
use std::{collections::HashMap, env, fs};
struct Config {
    file_name: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 2 {
            Err("No File name provided in the command line ".to_string())
        } else {
            println!("{args:?}");
            let name = args[1].clone();
            Ok(Config { file_name: name })
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap();
    let mut map: HashMap<String, u64> = HashMap::new();

    word_count(&config.file_name, &config.file_name, &mut map);

    println!("{map:?}");
}

fn word_count(key: &str, file_name: &str, map: &mut HashMap<String, u64>) {
    let content = fs::read_to_string(file_name).expect("Error reading file");
    let pattern = Regex::new(r"\s+").unwrap();

    let words: Vec<&str> = pattern.split(&content).collect();

    map.insert(key.to_string(), words.len() as u64);
}

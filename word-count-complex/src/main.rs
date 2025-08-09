use regex::Regex;
use std::{
    collections::HashMap,
    env, fs, io,
    sync::{Arc, Mutex},
    thread,
};
struct Config {
    dir_name: String,
    // is_dir : String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 2 {
            Err("No File name provided in the command line ".to_string())
        } else {
            let name = args[1].clone();
            Ok(Config { dir_name: name })
        }
    }
}

static THREAD_COUNT: i32 = 10;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap();
    let mut files: Vec<String> = Vec::new();

    if let Err(e) = list_files(&config.dir_name, &mut files) {
        println!("Error {}", e);
    }

    let total_word_count = Arc::new(Mutex::new(0));
    let shared_files = Arc::new(files);
    let map: Arc<Mutex<HashMap<String, u64>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for i in 0..THREAD_COUNT {
        let shared_files = Arc::clone(&shared_files);
        let shared_map = Arc::clone(&map);
        let shared_word_count = Arc::clone(&total_word_count);
        let files_per_thread = shared_files.len() as i32 / THREAD_COUNT;
        let handle = thread::spawn(move || {
            let start = (i * files_per_thread) as usize;
            let end = ((i + 1) * files_per_thread) as usize;
            for file in &shared_files[start..end] {
                let count = word_count(file);
                let mut global_count = shared_word_count.lock().unwrap();
                *global_count += count;

                let mut map = shared_map.lock().unwrap();
                map.insert(file.to_string(), count);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Count : {}", total_word_count.lock().unwrap());
    println!("Map : {:?}", map.lock().unwrap());
}

fn word_count(file_name: &str) -> u64 {
    let content = fs::read_to_string(file_name).expect("Error reading file");
    let pattern = Regex::new(r"\s+").unwrap();

    let words: Vec<&str> = pattern.split(&content).collect();

    words.len() as u64
}

fn list_files(dir: &str, files: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                files.push(path_str.to_string());
            }
        } else if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                list_files(path_str, files)?;
            }
        }
    }
    Ok(())
}

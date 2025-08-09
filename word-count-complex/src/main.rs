use std::{
    cmp::min,
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

const THREAD_COUNT: usize = 10;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let mut files: Vec<String> = Vec::new();

    if let Err(e) = list_files(&config.dir_name, &mut files) {
        println!("Error {}", e);
    }

    if files.is_empty() {
        println!("Directory is empty");
        return;
    }

    let total_word_count = Arc::new(Mutex::new(0));
    let shared_files = Arc::new(files);
    let map: Arc<Mutex<HashMap<String, u64>>> = Arc::new(Mutex::new(HashMap::new()));
    let len = shared_files.len();
    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(THREAD_COUNT);
    let files_per_thread = (len + thread_count - 1) / thread_count;

    let mut handles = Vec::new();

    for i in 0..thread_count {
        let shared_files = Arc::clone(&shared_files);
        let shared_map = Arc::clone(&map);
        let shared_word_count = Arc::clone(&total_word_count);

        let start = (i * files_per_thread) as usize;

        if start >= len {
            break;
        }

        let end = min(len, start + files_per_thread);

        let handle = thread::spawn(move || {
            let mut local_count: u64 = 0;
            let mut local_map: Vec<(String, u64)> = Vec::new();
            for file in &shared_files[start..end] {
                let count = word_count(file);
                local_count += count;

                local_map.push((file.to_string(), count));
            }

            let mut global_count = shared_word_count.lock().unwrap();
            *global_count += local_count;

            let mut map = shared_map.lock().unwrap();
            for entry in local_map {
                map.insert(entry.0, entry.1);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Count : {}", *total_word_count.lock().unwrap());
    println!("Map : {:?}", map.lock().unwrap());
}

fn word_count(file_name: &str) -> u64 {
    let content = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_name, e);
            return 0;
        }
    };

    content.split_whitespace().count() as u64
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

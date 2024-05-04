use reqwest::blocking::Client;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path;

fn main() {
    let client = Client::new();
    for play_url in fs::read_to_string("./config/play_urls.txt")
        .unwrap()
        .lines()
    {
        match try_to_fetch_cached_response(play_url) {
            Some(response_text) => println!("Got cached hit for {play_url}"),
            None => {
                print!("Fetching {}", play_url);
                let result = client.get(play_url).send(); // TODO parallelize requests
                match result {
                    Ok(response) => {
                        let status = response.status();
                        println!(" -> {}", status);
                        if status.is_success() {
                            if let Ok(text) = response.text() {
                                cache_response(play_url, text);
                            } else {
                                println!("Failed to decode response for {}", play_url);
                            }
                        }
                    }
                    Err(error) => println!("Error for {} -> {}", play_url, error),
                }
            }
        }
    }
}

fn get_cache_file_path(play_url: &str) -> String {
    let mut sanitized_url = play_url.to_string();
    sanitized_url.retain(|c| c.is_ascii_alphanumeric());
    let mut hasher = DefaultHasher::new();
    play_url.hash(&mut hasher);
    format!("cache/{sanitized_url}_{:x}", hasher.finish())
}

fn try_to_fetch_cached_response(play_url: &str) -> Option<String> {
    let cache_file_path = get_cache_file_path(play_url);
    let file_exists = path::Path::new(&cache_file_path).exists();
    if file_exists {
        Some(fs::read_to_string(cache_file_path).unwrap())
    } else {
        None
    }
}

fn cache_response(play_url: &str, response_text: String) {
    let cache_file_path = get_cache_file_path(play_url);
    println!("Caching at {cache_file_path}");
    fs::write(cache_file_path, response_text).unwrap();
}

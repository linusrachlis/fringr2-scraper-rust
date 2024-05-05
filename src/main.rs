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
        if let Some(response_text) = get_response_text(&client, play_url) {
            println!("Goit response text {response_text}");
        } else {
            println!("Got no satisfaction");
        }
    }
}

fn get_response_text(client: &Client, play_url: &str) -> Option<String> {
    match try_to_fetch_cached_response(play_url) {
        Some(response_text) => Some(response_text),
        None => {
            // Fetch afresh
            let mut maybe_response_text: Option<String> = None;
            print!("Fetching {}", play_url);
            let result = client.get(play_url).send(); // TODO parallelize requests
            match result {
                Ok(response) => {
                    let status = response.status();
                    println!(" -> {}", status);
                    if status.is_success() {
                        if let Ok(response_text) = response.text() {
                            cache_response(play_url, &response_text);
                            maybe_response_text = Some(response_text);
                        } else {
                            println!("Failed to decode response for {}", play_url);
                        }
                    }
                }
                Err(error) => println!("Error for {} -> {}", play_url, error),
            }
            maybe_response_text
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

fn cache_response(play_url: &str, response_text: &String) {
    let cache_file_path = get_cache_file_path(play_url);
    println!("Caching at {cache_file_path}");
    fs::write(cache_file_path, response_text).unwrap();
}

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path;

fn main() {
    let client = Client::new();
    for play_url in fs::read_to_string("./config/play_urls.txt")
        .unwrap()
        .lines()
    {
        let play_page_html = get_response_text(&client, play_url).unwrap();
        let document = Html::parse_document(&play_page_html);
        let title_selector = Selector::parse(".page-title").unwrap();
        let title_text = document
            .select(&title_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let title_text = title_text.trim();
        println!("Title: [{title_text}]");

        let runtime_text_selector = Selector::parse(".show-info .column.right dd").unwrap();
        let maybe_runtime_text_container = document.select(&runtime_text_selector).next();
        if let Some(runtime_text_container) = maybe_runtime_text_container {
            let runtime_text = runtime_text_container.text().collect::<String>();
            let runtime_text = runtime_text.trim_start();

            // Get all the starting digits from the node's text; this is the number of minutes for
            // the runtime.
            let mut runtime_last_digit_position = 0;
            for c in runtime_text.chars() {
                if !c.is_ascii_digit() {
                    break;
                }
                runtime_last_digit_position += 1;
            }
            let runtime_minutes = &runtime_text[0..runtime_last_digit_position];
            println!("Runtime: [{runtime_minutes}]");
        } else {
            println!("No runtime");
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

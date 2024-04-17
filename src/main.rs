use reqwest::blocking::{Client, Response};
use std::fs::read_to_string;

// Simpler case for figuring out ownership stuff
// fn main() {
//     let client = Client::new();
//     let play_url = "https://fringetoronto.com/next-stage/show/black-canada";
//     print!("Fetching {}", play_url);
//     match client.get(play_url).send() {
//         Ok(response) => {
//             let status = response.status();
//             println!("-> {}", status);
//             if status.is_success() {
//                 if let Ok(text) = response.text() {
//                     println!("Here is the HTML content for {}", response.url());
//                     println!("----------------------------------------");
//                     println!("{text}");
//                 } else {
//                     println!("Failed to decode response for {}", response.url());
//                 }
//             }
//         }
//         Err(error) => println!("Error for {} -> {}", play_url, error),
//     }
// }

fn main() {
    let mut responses = Vec::<Response>::new();
    let client = Client::new();
    let mut num_play_urls = 0;
    for play_url in read_to_string("./config/play_urls.txt").unwrap().lines() {
        num_play_urls += 1;
        print!("Fetching {}", play_url);
        let result = client.get(play_url).send(); // TODO parallelize requests
        match result {
            Ok(response) => {
                let status = response.status();
                println!(" -> {}", status);
                if status.is_success() {
                    responses.push(response);
                }
            }
            Err(error) => println!("Error for {} -> {}", play_url, error),
        }
        break; // FIXME this is limiting to the first play for dev iteration
    }

    println!(
        "Successfully fetched {}/{} URLs",
        responses.len(),
        num_play_urls
    );

    for response in responses {
        let url = String::from(response.url().as_str());
        if let Ok(text) = response.text() {
            println!("Here is the HTML content for {}", url);
            println!("----------------------------------------");
            println!("{text}");
        } else {
            println!("Failed to decode response for {}", url);
        }
        return; // FIXME this is stopping the loop
    }
}

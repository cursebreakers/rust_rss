// main.rs

use reqwest::blocking::get;
use reqwest::Error;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::env;
use std::fs;
use std::path::Path;
use std::str;
use std::string::String;

#[derive(Deserialize)]
struct FeedList {
    rss_feeds: Vec<String>,
}

// Function to fetch and parse RSS feed
fn fetch_rss_feed(url: &str) -> Result<String, Error> {
    println!("Fetching: {}", url);
    let response = get(url)?.text()?;
    Ok(response)
}

// Extract content between start_tag and end_tag
fn extract_element(content: &str, start_tag: &str, end_tag: &str) -> Option<String> {
    if let Some(start) = content.find(start_tag) {
        let start_pos = start + start_tag.len();
        if let Some(end) = content[start_pos..].find(end_tag) {
            return Some(content[start_pos..start_pos + end].to_string());
        }
    }
    None
}

// Parse and print RSS feed
fn parse_rss_feed(rss_content: &str, fetch_all: bool) {
    let item_start = "<item>";
    let item_end = "</item>";
    let mut pos = 0;

    while let Some(start) = rss_content[pos..].find(item_start) {
        let start_pos = pos + start;
        if let Some(end) = rss_content[start_pos..].find(item_end) {
            let end_pos = start_pos + end + item_end.len();
            let item_content = &rss_content[start_pos..end_pos];

            let title = extract_element(item_content, "<title>", "</title>").unwrap_or_default();
            let link = extract_element(item_content, "<link>", "</link>").unwrap_or_default();
            let pub_date_str = extract_element(item_content, "<pubDate>", "</pubDate>").unwrap_or_default();

            if !title.is_empty() && !link.is_empty() && !pub_date_str.is_empty() {
                if fetch_all || is_today(&pub_date_str) {
                    println!("Headline: {}", title);
                    println!("URL: {}", link);
                    println!("Publication Date: {}\n", pub_date_str);
                }
            }
            pos = end_pos; // Move past the processed <item>
        } else {
            break; // No more items found
        }
    }
}


// Read feeds from feeds.json
fn read_feeds_from_json(file_path: &str) -> Vec<String> {
    // Check if the file exists
    if !Path::new(file_path).exists() {
        eprintln!("Error: {} does not exist!", file_path);
        std::process::exit(1);
    }

    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let feed_list: FeedList = serde_json::from_str(&data).expect("Unable to parse JSON");

    feed_list.rss_feeds
}

fn is_today(pub_date_str: &str) -> bool {
    if let Ok(pub_date) = DateTime::parse_from_rfc2822(pub_date_str) {
        pub_date.date_naive() == Utc::now().date_naive()
    } else {
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fetch_all = args.contains(&"-a".to_string()) || args.contains(&"--all".to_string());
    let fetch_today = args.contains(&"-t".to_string()) || args.contains(&"--today".to_string());

    let rss_feeds = read_feeds_from_json("src/feeds.json");

    for feed in rss_feeds {
        match fetch_rss_feed(&feed) {
            Ok(rss_content) => parse_rss_feed(&rss_content, fetch_all || fetch_today),
            Err(e) => eprintln!("Error fetching RSS feed {}: {}", feed, e),
        }
    }
}


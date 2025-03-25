// main.rs

use reqwest::blocking::get;
use reqwest::Error;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::env;
use std::str;
use std::string::String;
use colored::*;

// Function to fetch and parse RSS feed
fn fetch_rss_feed(url: &str) -> Result<(String, String), Error> {
    println!("Fetching: {}", url.green());
    let response = get(url)?.text()?;
    Ok((url.to_string(), response))
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
fn parse_rss_feed(feed_url: &str, rss_content: &str, fetch_all: bool) {
    let item_start_rss = "<item>";
    let item_end_rss = "</item>";
    let entry_start_atom = "<entry>";
    let entry_end_atom = "</entry>";
    
    let mut pos = 0;
    let mut found_posts = false;

    let is_rss = rss_content.contains("<item>");
    let is_atom = rss_content.contains("<entry>");

    if is_rss {
        while let Some(start) = rss_content[pos..].find(item_start_rss) {
            let start_pos = pos + start;
            if let Some(end) = rss_content[start_pos..].find(item_end_rss) {
                let end_pos = start_pos + end + item_end_rss.len();
                let item_content = &rss_content[start_pos..end_pos];

                let title = extract_element(item_content, "<title>", "</title>").unwrap_or_default();
                let link = extract_element(item_content, "<link>", "</link>").unwrap_or_default();
                let pub_date_str = extract_element(item_content, "<pubDate>", "</pubDate>").unwrap_or_default();

                if !title.is_empty() && !link.is_empty() && !pub_date_str.is_empty() {
                    if fetch_all || is_today(&pub_date_str) {
                        println!("{}: {}", "Headline".truecolor(255, 255, 255).bold(), title.truecolor(255, 255, 255).bold());

                        println!("{}: {}", "URL".truecolor(0, 255, 255), link.truecolor(150, 220, 210));
                        println!("{}: {}\n", "Publication Date".truecolor(100, 50, 255), pub_date_str.truecolor(100, 50, 255));
                        found_posts = true;
                    }
                }
                pos = end_pos; // Move past the processed <item>
            } else {
                break;
            }
        }
    }

    if is_atom {
        while let Some(start) = rss_content[pos..].find(entry_start_atom) {
            let start_pos = pos + start;
            if let Some(end) = rss_content[start_pos..].find(entry_end_atom) {
                let end_pos = start_pos + end + entry_end_atom.len();
                let entry_content = &rss_content[start_pos..end_pos];

                let title = extract_element(entry_content, "<title>", "</title>").unwrap_or_default();
                let link = extract_element(entry_content, "<link>", "</link>").unwrap_or_default();
                let updated_str = extract_element(entry_content, "<updated>", "</updated>").unwrap_or_default();

                if !title.is_empty() && !link.is_empty() && !updated_str.is_empty() {
                    if fetch_all || is_today(&updated_str) {
                        println!("{}: {}", "Headline".truecolor(255, 255, 255), title.truecolor(0, 255, 0)); // Electric green
                        println!("{}: {}", "URL".truecolor(150, 220, 210), link.truecolor(0, 255, 255)); // Electric green
                        println!("{}: {}\n", "Updated Date".truecolor(100, 50, 255), updated_str.truecolor(100, 50, 255));
                        found_posts = true;
                    }
                }
                pos = end_pos; // Move past the processed <entry>
            } else {
                break;
            }
        }
    }

    if !found_posts {
	    println!("{} {}", "No posts retrieved from".truecolor(255, 255, 0), feed_url.truecolor(255, 255, 0));	
    }
}

// Read feeds from feeds.json
fn read_feeds_from_json(file_path: &str) -> Vec<String> {
    let mut file = File::open(file_path).expect("Failed to open feeds.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let json: Value = serde_json::from_str(&contents).expect("Invalid JSON format");

    let mut feeds = Vec::new();
    if let Some(rss_feeds) = json["rss_feeds"].as_array() {
        feeds.extend(rss_feeds.iter().filter_map(|f| f.as_str().map(String::from)));
    }
    if let Some(other_feeds) = json["other_feeds"].as_array() {
        feeds.extend(other_feeds.iter().filter_map(|f| f.as_str().map(String::from)));
    }

    feeds
}

// is it from today?
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

    let rss_feeds = read_feeds_from_json("./feeds.json");

    for feed in rss_feeds {
        match fetch_rss_feed(&feed) {
            Ok((feed_url, rss_content)) => parse_rss_feed(&feed_url, &rss_content, fetch_all || fetch_today),
            Err(e) => eprintln!("Error fetching RSS feed {}: {}", feed, e),
        }
    }
}


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
use std::collections::HashMap;
use colored::*;

const MAIN_FEEDS: bool = true;
const CYBERSECURITY_FEEDS: bool = true;
const SCIENCE_FEEDS: bool = true;
const FAVORITES_FEEDS: bool = true;

// Function to fetch and parse RSS feed
fn fetch_rss_feed(url: &str) -> Result<(String, String), Error> {
    println!("Fetching RSS feed from: {}", url.green());
    let response = get(url)?.text()?;
    Ok((url.to_string(), response))
}

// Extract content between start_tag and end_tag
fn extract_element(content: &str, start_tag: &str, end_tag: &str) -> Option<String> {
    if let Some(start) = content.find(start_tag) {
        let start_pos = start + start_tag.len();
        if let Some(end) = content[start_pos..].find(end_tag) {
            let mut extracted = content[start_pos..start_pos + end].to_string();
            extracted = extracted.replace("<![CDATA[", "").replace("]]>", ""); // Remove CDATA markers
            return Some(extracted);
        }
    }
    None
}

// Parse and print RSS feed
fn parse_rss_feed(feed_url: &str, rss_content: &str, fetch_all: bool) -> usize {
    let item_start_rss = "<item>";
    let item_end_rss = "</item>";
    let entry_start_atom = "<entry>";
    let entry_end_atom = "</entry>";

    let mut pos = 0;
    let mut post_count = 0;

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
                        post_count += 1;
                    }
                }
                pos = end_pos;
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
                        println!("{}: {}", "Headline".truecolor(255, 255, 255), title.truecolor(0, 255, 0));
                        println!("{}: {}", "URL".truecolor(150, 220, 210), link.truecolor(0, 255, 255));
                        println!("{}: {}\n", "Updated Date".truecolor(100, 50, 255), updated_str.truecolor(100, 50, 255));
                        post_count += 1;
                    }
                }
                pos = end_pos;
            } else {
                break;
            }
        }
    }

    if post_count == 0 {
        println!("{} {}", "No posts retrieved from".truecolor(255, 255, 0), feed_url.truecolor(255, 255, 0));
    } else {
        println!("Retrieved {} posts from {}\n", post_count, feed_url.green());
    }

    post_count
}

// Read feeds from feeds.json
fn read_feeds_from_json(file_path: &str) -> Vec<String> {
    let mut file = File::open(file_path).expect("Failed to open feeds.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let json: Value = serde_json::from_str(&contents).expect("Invalid JSON format");

    let mut feeds = Vec::new();
    if MAIN_FEEDS {
		println!("Getting main feeds...");
        if let Some(rss_feeds) = json["main_feeds"].as_array() {
            feeds.extend(rss_feeds.iter().filter_map(|f| f.as_str().map(String::from)));
        }
    }
    if CYBERSECURITY_FEEDS {
		println!("Getting cybersecurity feeds...");
        if let Some(rss_feeds) = json["cybersecurity"].as_array() {
            feeds.extend(rss_feeds.iter().filter_map(|f| f.as_str().map(String::from)));
        }
    }
    if SCIENCE_FEEDS {
		println!("Getting science feeds...");
        if let Some(rss_feeds) = json["science"].as_array() {
            feeds.extend(rss_feeds.iter().filter_map(|f| f.as_str().map(String::from)));
        }
    }
    if FAVORITES_FEEDS {
		println!("Getting favorite feeds...");
        if let Some(rss_feeds) = json["favorites"].as_array() {
            feeds.extend(rss_feeds.iter().filter_map(|f| f.as_str().map(String::from)));
        }
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
    println!("Fetching from {} feeds...\n", rss_feeds.len());

    let mut total_posts = 0;
    let mut feed_counts: HashMap<String, usize> = HashMap::new();

    for feed in rss_feeds {
    let post_count = match fetch_rss_feed(&feed) {
        Ok((feed_url, rss_content)) => parse_rss_feed(&feed_url, &rss_content, fetch_all || fetch_today),
        Err(e) => {
            eprintln!("Error fetching RSS feed {}: {}", feed, e);
            0
        }
    };

    total_posts += post_count;
    feed_counts.insert(feed, post_count);
}

    println!("\nSummary:");
for (feed, count) in &feed_counts {
    if *count == 0 {
        println!("{}: {} posts", feed.red(), count.to_string().bold().red());
    } else {
        println!("{}: {} posts", feed.green(), count.to_string().bold());
    }
}
    
    println!("\nTotal posts retrieved: {}", total_posts.to_string().bold());
}


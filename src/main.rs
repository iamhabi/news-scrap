use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

use reqwest;

use soup::prelude::*;

use serde::{Serialize, Deserialize};

use std::time::{Duration, UNIX_EPOCH};

use mysql::chrono::prelude::*;

mod sql;

#[derive(Debug, Serialize, Deserialize)]
struct News {
    title: String,
    href: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct From<'v> {
    url: &'v str,
    tag: &'v str,
    attr: (&'v str, &'v str),
}

#[tokio::main]
async fn main() {
    loop {
        let news = get_news().await;

        for n in news {
            sql::insert_to_sql(&n.title, &n.href);
        }

        print_time();

        std::thread::sleep(Duration::from_millis(1000 * 60 * 60));
    }
}

fn print_time() {
    let timestamp: u64 = Utc::now().timestamp().try_into().unwrap();
    let d = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = DateTime::<Utc>::from(d);
    println!("{}", datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string());
}

fn check_blacklist(title: &str, href: &str) -> bool {
    let black = ["best", "black-friday", "View All Stories", "good-deals", "Vergecast"];

    for b in black {
        if title.contains(b) || href.contains(b) {
            return true;
        }
    }

    false
}

fn check_href(href: &str) -> bool {
    href.contains("http")
}

async fn get_news() -> Vec<News> {
    let mut f = read_json();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Error read to string");

    let from: Vec<From> = serde_json::from_str(&buffer).unwrap();

    let mut news = Vec::new();

    for f in from {
        let result = reqwest::get(f.url).await.unwrap();
        let body = result.text().await.unwrap();
        let soup = Soup::new(&body);

        for (_j, link) in soup.tag(f.tag).attr(f.attr.0, f.attr.1).find_all().enumerate() {
            let title = link.text();

            let mut href = link.get("href").expect("Couldn't find link with 'href' attribute");

            if check_blacklist(&title, &href) {
                continue;
            }

            if check_href(&href) == false {
                href = f.url.to_owned() + &href;
            }

            let n = News {
                title: title,
                href: href,
            };

            news.push(n);
        }
    }

    news
}

fn read_json() -> File {
    File::open("read.json").expect("Error while open file")
}
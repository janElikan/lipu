use std::fs;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use lipu::{Article, ArticleBody};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut feeds = vec![
        Feed::new("https://www.0atman.com/feed.xml"),
        Feed::new("https://www.spreaker.com/show/4488937/episodes/feed"), // LT
        Feed::new("https://www.spreaker.com/show/6029902/episodes/feed"), // TPC
        Feed::new("https://www.youtube.com/feeds/videos.xml?channel_id=UCUMwY9iS8oMyWDYIe6_RmoA"), // NB
    ];

    refresh_feeds(&mut feeds).await?;

    let mut articles: Vec<_> = feeds.into_iter().flat_map(|feed| feed.articles).collect();
    articles.sort_by(|a, b| b.created.partial_cmp(&a.created).expect("sorting error?"));

    loop {
        let selected = inquire::Select::new("Select an article you want to view", articles.clone())
            .prompt()?;

        match &selected.body {
            ArticleBody::Text(text) => println!("#{}\n{}", selected.name, text),
            ArticleBody::Audio(payload) => {
                if !confirm_download()? {
                    continue;
                }

                let path = format!("/tmp/{}", selected.name);
                let stream = reqwest::get(&payload.url).await?.bytes().await?;

                fs::write(path, stream)?;
            }
            ArticleBody::YouTubeLink(link) => println!("\n\n{link}\n\n"),
            ArticleBody::Video(_payload) => {
                todo!()
            }
        }
    }
}

#[derive(Debug)]
struct Feed {
    url: String,
    articles: Vec<Article>,
    last_updated: DateTime<Utc>,
}

impl Feed {
    fn new(url: &str) -> Self {
        Self {
            url: String::from(url),
            articles: Vec::new(),
            last_updated: DateTime::UNIX_EPOCH,
        }
    }
}

async fn refresh_feeds(feeds: &mut Vec<Feed>) -> Result<()> {
    for feed in feeds {
        feed.articles = fetch(&feed.url).await?;
        feed.last_updated = Utc::now();
    }

    Ok(())
}

async fn fetch(feed_url: &str) -> Result<Vec<lipu::Article>> {
    let xml = reqwest::get(feed_url).await?.text().await?;
    let feed = feed_rs::parser::parse(xml.as_bytes())?;

    Ok(feed
        .entries
        .into_iter()
        .flat_map(lipu::Article::try_from)
        .collect())
}

fn confirm_download() -> Result<bool> {
    Ok(inquire::Confirm::new("Download?")
        .with_default(true)
        .prompt()?)
}

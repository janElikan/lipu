use std::fmt::Display;

pub mod core;

use core::{Article, ArticleBody, Progress};

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self.viewed {
            Progress::None => "[new]    ",
            Progress::Fully => "[viewed] ",
            _ => "[partial]",
        };

        let article_type = match self.body {
            ArticleBody::Text(_) => "(article)",
            ArticleBody::Audio(_) => "(audio)",
            ArticleBody::Video(_) => "(video)",
            ArticleBody::YouTubeLink(_) => "(youtube video)",
        };

        write!(f, "{} {} {}", status, self.name, article_type)
    }
}

#[derive(Debug)]
enum Error {
    NetworkError,
    FetchTextFailed,
    FeedParsingFailed,
}

async fn fetch(feed_url: &str) -> Result<Vec<Article>, Error> {
    let xml = reqwest::get(feed_url)
        .await
        .map_err(|_| Error::NetworkError)?
        .text()
        .await
        .map_err(|_| Error::FetchTextFailed)?;

    let feed = feed_rs::parser::parse(xml.as_bytes()).map_err(|_| Error::FeedParsingFailed)?;

    Ok(feed
        .entries
        .into_iter()
        .map(Article::try_from)
        .map(|result| match result {
            Ok(data) => Ok(data),
            Err(why) => {
                println!("download item failed: `{:?}`", why);
                Err(why)
            }
        })
        .flatten()
        .collect())
}

#[derive(Default, Clone)]
pub struct App {
    feed_urls: Vec<String>,
    merge_with: Option<Vec<Article>>,
}

impl App {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            feed_urls: Vec::new(),
            merge_with: None,
        }
    }

    #[must_use]
    pub fn add_feed(mut self, url: &str) -> Self {
        self.feed_urls.push(url.to_string());

        self
    }

    #[must_use]
    pub fn copy_statuses(mut self, old_articles: Vec<Article>) -> Self {
        self.merge_with = Some(old_articles);

        self
    }

    pub async fn fetch(&self) -> Vec<Article> {
        let mut feeds = Vec::new();

        for url in &self.feed_urls {
            println!("downloading feed {url}");
            feeds.push(fetch(url).await);
        }

        let mut feeds: Vec<Article> = feeds
            .into_iter()
            .map(|result| match result {
                Ok(data) => Ok(data),
                Err(why) => {
                    println!("download feed failed: `{:?}`", why);
                    Err(why)
                }
            })
            .flatten()
            .flatten()
            .map(|article| {
                if let Some(others) = &self.merge_with {
                    let Some(other) = others.iter().find(|candidate| candidate.id == article.id)
                    else {
                        return article;
                    };

                    Article {
                        viewed: other.viewed.clone(),
                        ..article
                    }
                } else {
                    article
                }
            })
            .collect();

        feeds.sort_by(|a, b| b.created.partial_cmp(&a.created).expect("sorting failed"));

        feeds
    }
}

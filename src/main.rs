use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use lipu::{Article, ArticleBody};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut feeds = vec![
        Feed::new("https://www.0atman.com/feed.xml"),
        Feed::new("https://www.spreaker.com/show/6029902/episodes/feed"),
    ];

    refresh_feeds(&mut feeds).await?;

    let mut articles: Vec<_> = feeds.into_iter().flat_map(|feed| feed.articles).collect();
    articles.sort_by(|a, b| a.created.partial_cmp(&b.created).expect("sorting error?"));

    let selected = inquire::Select::new("Select an article you want to view", articles).prompt()?;
    match selected.body {
        ArticleBody::Text(text) => println!("#{}\n{}", selected.name, text),
        ArticleBody::Audio(payload) => todo!(),
        ArticleBody::Video(payload) => todo!(),
    }

    Ok(())
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
    let xml = reqwest::get(feed_url).await?.bytes().await?;
    let feed = rss::Channel::read_from(&xml[..])?;

    Ok(feed
        .items
        .into_iter()
        .flat_map(lipu::Article::try_from)
        .collect())
}

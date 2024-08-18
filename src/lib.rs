use std::path::PathBuf;

use chrono::{DateTime, Utc};

pub struct Lipu {
    feeds: Vec<String>,
    items: Vec<Item>,
}

pub enum Error {
    NoNetwork,
    ParsingFailed,
    NotFound,
}

pub trait LipuInterface {
    fn add_feed(&mut self, url: String);
    fn add_mastodon_feed(&mut self, instance: String, user: String);
    fn add_youtube_channel(&mut self, channel_id: String);
    async fn refresh(&mut self) -> Result<(), Error>;
    fn remove_feed(&mut self, url: &str) -> Result<(), Error>;

    fn list(&self) -> &[Metadata];
    fn search(&self, query: &str) -> &[Metadata];
    fn with_tag(&self) -> &[Metadata];

    fn add_tag(&mut self, item_id: &str, tag: &str) -> Result<(), ()>;
    fn remove_tag(&mut self, item_id: &str, tag: &str) -> Result<(), ()>;
    fn drop_tag(&mut self, tag: &str) -> Result<(), ()>;

    fn load(&self, item_id: &str) -> &Item;
    fn set_viewing_progress(&mut self, item_id: &str, progress: ViewingProgress) -> Result<(), ()>;
    async fn download_item(&mut self, item_id: &str) -> Result<(), ()>;
}

impl Lipu {
    fn new() -> Self {
        Self {
            feeds: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl LipuInterface for Lipu {
    fn add_feed(&mut self, url: String) {
        self.feeds.push(url);
    }

    fn add_mastodon_feed(&mut self, instance: String, user: String) {
        let url = format!("https://{instance}/@{user}.rss");
        self.feeds.push(url);
    }

    fn add_youtube_channel(&mut self, channel_id: String) {
        let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}");
        self.feeds.push(url);
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        let old_item_ids: Vec<_> = self.items.iter().map(|item| &item.metadata.id).collect();

        let mut feeds = Vec::new();
        for url in &self.feeds {
            let xml = reqwest::get(url)
                .await
                .map_err(|_| Error::NoNetwork)?
                .text()
                .await
                .map_err(|_| Error::ParsingFailed)?;

            feeds.push((url, xml));
        }

        let new_items: Vec<_> = feeds
            .into_iter()
            .map(|(url, xml)| (url, feed_rs::parser::parse(xml.as_bytes())))
            .filter(|(_, feed)| feed.is_ok())
            .map(|(url, feed)| (url, feed.unwrap()))
            .flat_map(|(url, feed)| feed.entries.into_iter().map(|entry| Item::from(entry, url)))
            .filter(|item| !old_item_ids.contains(&&item.metadata.id))
            .collect();

        new_items.into_iter().for_each(|item| self.items.push(item));

        Ok(())
    }

    fn remove_feed(&mut self, url: &str) -> Result<(), Error> {
        let (idx, _) = self
            .feeds
            .iter()
            .enumerate()
            .find(|(_, feed)| *feed == url)
            .ok_or(Error::NotFound)?;

        self.feeds.remove(idx);

        let item_indexes: Vec<_> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.metadata.feed_url == url)
            .map(|(idx, _)| idx)
            .rev()
            .collect();

        item_indexes.into_iter().for_each(|idx| {
            self.items.remove(idx);
        });

        Ok(())
    }
}

pub struct Item {
    pub metadata: Metadata,
    pub body: Body,
}

pub struct Metadata {
    pub id: String,

    pub name: String,
    pub tags: Vec<String>,

    pub feed_url: String,
    pub link: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,

    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,

    pub viewed: ViewingProgress,
}

pub enum Body {
    DownloadLink { mime_type: String, url: String },
    File { mime_type: String, path: PathBuf },
}

pub enum ViewingProgress {
    Zero,
    UntilParagraph(usize),
    UntilSecond(usize),
    Fully,
}

impl Item {
    fn from(entry: feed_rs::model::Entry, feed_url: &str) -> Self {
        todo!();
    }
}

/*
impl TryFrom<feed_rs::model::Entry> for Item {
    type Error = ArticleCreationError;

    fn try_from(entry: feed_rs::model::Entry) -> Result<Self, Self::Error> {
        let summary = match entry.summary {
            Some(text) => Some(text.content),
            None => None,
        };

        let body = if entry.media.is_empty() {
            let text = match entry.content {
                Some(content) => content.body,
                None => None,
            };

            Body::Html(text.unwrap_or(summary.clone().ok_or(Self::Error::EmptyBody)?))
        } else {
            let media = entry
                .media
                .into_iter()
                .next()
                .expect("just checked that it had media and now it doesn't");

            // I haven't seen anyone attach more than one media item...
            let media = media
                .content
                .into_iter()
                .next()
                .ok_or(Self::Error::EmptyContent)?;

            let payload = MediaLink {
                url: media
                    .url
                    .ok_or(Self::Error::MissingDownloadUrl)?
                    .to_string(),
                mime_type: media
                    .content_type
                    .ok_or(Self::Error::UnknownMimeType)?
                    .to_string(),
                downloaded: false,
            };

            match payload
                .mime_type
                .split_once('/')
                .ok_or(Self::Error::UnknownMimeType)?
            {
                ("application", "x-shockwave-flash") => Body::YouTubeLink(payload.url),
                ("video", _) => Body::Video(payload),
                ("audio", _) => Body::Audio(payload),
                _ => return Err(Self::Error::UnknownMimeType),
            }
        };

        Ok(Self {
            id: entry.id,
            name: match entry.title {
                Some(text) => text.content,
                None => "??".to_string(),
            },
            source: entry.source,
            author: {
                if entry.authors.is_empty() {
                    None
                } else {
                    let authors = entry
                        .authors
                        .into_iter()
                        .map(|author| author.name)
                        .collect::<Vec<String>>()
                        .join(", ");

                    Some(authors)
                }
            },
            description: summary,
            created: entry.published,
            updated: entry.updated,
            viewed: ViewingProgress::Zero,
            body,
        })
    }
}
*/

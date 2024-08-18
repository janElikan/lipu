use std::path::PathBuf;

use chrono::{DateTime, Utc};
use tokio::{fs, io::AsyncWriteExt};

pub struct Lipu {
    feeds: Vec<String>,
    items: Vec<Item>,
    downloads_path: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    NoNetwork,
    CorruptedData,
    NotFound,
    CreateFileFailed,
    WriteFileFailed,
}

impl Lipu {
    pub fn new() -> Self {
        Self {
            feeds: Vec::new(),
            items: Vec::new(),
            downloads_path: "./downloads".into(),
        }
    }

    pub fn add_feed(&mut self, url: String) {
        self.feeds.push(url);
    }

    pub fn add_mastodon_feed(&mut self, instance: String, user: String) {
        let url = format!("https://{instance}/@{user}.rss");
        self.feeds.push(url);
    }

    pub fn add_youtube_channel(&mut self, channel_id: String) {
        let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}");
        self.feeds.push(url);
    }

    pub async fn refresh(&mut self) -> Result<(), Error> {
        let old_item_ids: Vec<_> = self.items.iter().map(|item| &item.metadata.id).collect();

        let mut feeds = Vec::new();
        for url in &self.feeds {
            let xml = reqwest::get(url)
                .await
                .map_err(|_| Error::NoNetwork)?
                .text()
                .await
                .map_err(|_| Error::CorruptedData)?;

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

    pub fn remove_feed(&mut self, url: &str) -> Result<(), Error> {
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

    pub fn list(&self) -> Vec<Metadata> {
        self.items
            .iter()
            .map(|item| item.metadata.clone())
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<Metadata> {
        self.items
            .iter()
            .filter(|item| {
                item.metadata.name.contains(query)
                    || item
                        .metadata
                        .author
                        .as_ref()
                        .is_some_and(|author| author.contains(query))
                    || item
                        .metadata
                        .tags
                        .iter()
                        .find(|tag| tag.contains(query))
                        .is_some()
            })
            .map(|item| item.metadata.clone())
            .collect()
    }

    pub fn with_tag(&self, tag: &str) -> Vec<Metadata> {
        self.items
            .iter()
            .filter(|item| {
                item.metadata
                    .tags
                    .iter()
                    .find(|potential| *potential == tag)
                    .is_some()
            })
            .map(|item| item.metadata.clone())
            .collect()
    }

    pub fn add_tag(&mut self, item_id: &str, tag: &str) -> Result<(), Error> {
        self.items
            .iter_mut()
            .find(|item| item.metadata.id == item_id)
            .ok_or(Error::NotFound)?
            .metadata
            .tags
            .push(tag.to_string());

        Ok(())
    }

    pub fn remove_tag(&mut self, item_id: &str, tag: &str) -> Result<(), Error> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.metadata.id == item_id)
            .ok_or(Error::NotFound)?;

        let (idx, _) = item
            .metadata
            .tags
            .iter()
            .enumerate()
            .find(|(_, potential)| *potential == tag)
            .ok_or(Error::NotFound)?;

        item.metadata.tags.remove(idx);

        Ok(())
    }

    pub fn drop_tag(&mut self, tag: &str) -> Result<(), Error> {
        let items_with_tag: Vec<_> = self
            .items
            .iter()
            .filter(|item| {
                item.metadata
                    .tags
                    .iter()
                    .find(|potential| *potential == tag)
                    .is_some()
            })
            .map(|item| item.metadata.id.clone())
            .collect();

        items_with_tag
            .into_iter()
            .map(|idx| self.remove_tag(&idx, tag))
            .collect()
    }

    pub fn load(&self, item_id: &str) -> Option<Item> {
        self.items
            .iter()
            .find(|item| item.metadata.id == item_id)
            .cloned()
    }

    pub fn set_viewing_progress(
        &mut self,
        item_id: &str,
        progress: ViewingProgress,
    ) -> Result<(), Error> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.metadata.id == item_id)
            .ok_or(Error::NotFound)?;

        item.metadata.viewed = progress;

        Ok(())
    }

    pub async fn download_item(&mut self, item_id: &str) -> Result<(), Error> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.metadata.id == item_id)
            .ok_or(Error::NotFound)?;

        match &item.body {
            Body::File { .. } => Ok(()),
            Body::Empty => Ok(()),
            Body::DownloadLink { mime_type, url } => {
                let bytes = reqwest::get(url)
                    .await
                    .map_err(|_| Error::NoNetwork)?
                    .bytes()
                    .await
                    .map_err(|_| Error::CorruptedData)?;

                let mut path = self.downloads_path.clone();
                path.push(item.metadata.id.clone());

                fs::File::create(path.clone())
                    .await
                    .map_err(|_| Error::CreateFileFailed)?
                    .write_all(&bytes)
                    .await
                    .map_err(|_| Error::WriteFileFailed)?;

                item.body = Body::File {
                    mime_type: mime_type.to_string(),
                    path,
                };

                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub metadata: Metadata,
    pub body: Body,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Body {
    DownloadLink { mime_type: String, url: String },
    File { mime_type: String, path: PathBuf },
    Empty,
}

#[derive(Clone, Debug)]
pub enum ViewingProgress {
    Zero,
    UntilParagraph(usize),
    UntilSecond(usize),
    Fully,
}

impl Item {
    fn from(entry: feed_rs::model::Entry, feed_url: &str) -> Self {
        let metadata = Metadata {
            name: match entry.title {
                Some(title) => title.content,
                None => entry.id.clone(),
            },
            id: entry.id,
            tags: Vec::new(),
            feed_url: feed_url.to_string(),
            link: entry.source,
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
            description: match entry.summary {
                Some(text) => Some(text.content),
                None => None,
            },
            created: entry.published,
            updated: entry.updated,
            viewed: ViewingProgress::Zero,
        };

        let body = match entry.media.first() {
            Some(body) => match body.content.first() {
                Some(data) => match (&data.content_type, &data.url) {
                    (Some(mime_type), Some(url)) => Body::DownloadLink {
                        mime_type: mime_type.to_string(),
                        url: url.to_string(),
                    },
                    _ => Body::Empty,
                },
                None => Body::Empty,
            },
            None => Body::Empty,
        };

        Self { metadata, body }
    }
}

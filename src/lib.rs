use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct Lipu {
    feeds: Vec<String>,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    NoNetwork,
    CorruptedData,
    NotFound,
    CreateFileFailed,
    WriteFileFailed,
}

pub struct Feed {
    name: String,
    url: String,
    body: String,
}

pub fn parse_feed(feed: Feed) -> Vec<Item> {
    let Ok(data) = feed_rs::parser::parse(feed.body.as_bytes()) else {
        return Vec::new();
    };

    let thumbnail = match data.logo {
        Some(logo) => Some(logo.uri),
        None => None,
    };

    data.entries
        .into_iter()
        .map(move |entry| Item::from(entry, &feed.url, thumbnail.clone()))
        .collect()
}

impl Lipu {
    /// ## Warning
    /// This is a blocking function: attempts to read from fs
    pub fn new() -> Self {
        Self {
            feeds: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn add_feed(&mut self, url: String) {
        if self.feeds.contains(&url) {
            return;
        };

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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub metadata: Metadata,
    pub body: Resource,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub id: String,

    pub name: String,
    pub tags: Vec<String>,

    pub feed_url: String,
    pub link: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,

    pub thubmnail: Option<Resource>,

    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,

    pub viewed: ViewingProgress,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Resource {
    Link {
        mime_type: Option<String>,
        url: String,
    },
    Missing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ViewingProgress {
    Zero,
    UntilParagraph(usize),
    UntilSecond(usize),
    Fully,
}

impl Item {
    fn from(entry: feed_rs::model::Entry, feed_url: &str, feed_thumbnail: Option<String>) -> Self {
        let feed_thumbnail = match feed_thumbnail {
            Some(url) => Some(Resource::Link {
                mime_type: None,
                url: url.to_string(),
            }),
            None => None,
        };

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
            thubmnail: match entry.media.first() {
                Some(media) => match media.thumbnails.first() {
                    Some(thumbnail) => Some(Resource::Link {
                        mime_type: None,
                        url: thumbnail.image.uri.clone(),
                    }),
                    None => feed_thumbnail,
                },
                None => feed_thumbnail,
            },
            created: entry.published,
            updated: entry.updated,
            viewed: ViewingProgress::Zero,
        };

        let body = match entry.media.first() {
            Some(body) => match body.content.first() {
                Some(data) => match (&data.content_type, &data.url) {
                    (Some(mime_type), Some(url)) => Resource::Link {
                        mime_type: Some(mime_type.to_string()),
                        url: url.to_string(),
                    },
                    (None, Some(url)) => Resource::Link {
                        mime_type: None,
                        url: url.to_string(),
                    },
                    _ => Resource::Missing,
                },
                None => Resource::Missing,
            },
            None => Resource::Missing,
        };

        Self { metadata, body }
    }
}

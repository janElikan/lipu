use std::fmt::Display;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Article {
    pub name: String,
    pub source: Option<String>,
    pub description: Option<String>,
    pub body: ArticleBody,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub viewed: Progress,
}

#[derive(Debug, Clone)]
pub enum ArticleBody {
    Text(String),
    Video(MediaPayload),
    Audio(MediaPayload),
    YouTubeLink(String),
}

#[derive(Debug, Clone)]
pub enum Progress {
    None,
    UntilLine(usize),
    UntilSecond(usize),
    Fully,
}

#[derive(Debug, Clone)]
pub struct MediaPayload {
    pub url: String,
    pub mime_type: String,
    pub downloaded: bool,
}

#[derive(Debug)]
pub enum ArticleCreationError {
    UnknownMimeType,
    EmptyBody,
    EmptyContent,
    MissingDownloadUrl,
}

impl TryFrom<feed_rs::model::Entry> for Article {
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

            ArticleBody::Text(text.unwrap_or(summary.clone().ok_or(Self::Error::EmptyBody)?))
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

            let payload = MediaPayload {
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
                ("application", "x-shockwave-flash") => ArticleBody::YouTubeLink(payload.url),
                ("video", _) => ArticleBody::Video(payload),
                ("audio", _) => ArticleBody::Audio(payload),
                _ => return Err(Self::Error::UnknownMimeType),
            }
        };

        Ok(Self {
            name: match entry.title {
                Some(text) => text.content,
                None => "??".to_string(),
            },
            source: entry.source,
            description: summary,
            created: entry.published,
            updated: entry.updated,
            viewed: Progress::None,
            body,
        })
    }
}

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

use std::fmt::Display;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Article {
    pub name: String,
    pub source: Option<String>,
    pub description: Option<String>,
    pub body: ArticleBody,
    pub created: Option<DateTime<Utc>>,
    pub viewed: Progress,
}

#[derive(Debug)]
pub enum ArticleBody {
    Text(String),
    Video(MediaPayload),
    Audio(MediaPayload),
}

#[derive(Debug)]
pub enum Progress {
    None,
    UntilLine(usize),
    UntilSecond(usize),
    Fully,
}

#[derive(Debug)]
pub struct MediaPayload {
    pub url: String,
    pub mime_type: String,
    pub downloaded: bool,
}

#[derive(Debug)]
pub enum ArticleCreationError {
    UnknownMimeType,
    EmptyBody,
}

impl TryFrom<rss::Item> for Article {
    type Error = ArticleCreationError;

    fn try_from(item: rss::Item) -> Result<Self, Self::Error> {
        let body = if let Some(media) = item.enclosure {
            match media
                .mime_type
                .split_once('/')
                .ok_or(Self::Error::UnknownMimeType)?
                .0
            {
                "audio" => ArticleBody::Audio(media.into()),
                "video" => ArticleBody::Video(media.into()),
                _ => todo!(),
            }
        } else {
            let content = item
                .content
                .unwrap_or(item.description.clone().ok_or(Self::Error::EmptyBody)?);
            ArticleBody::Text(content)
        };

        Ok(Self {
            name: item.title.unwrap_or(
                item.description
                    .as_ref()
                    .ok_or(Self::Error::EmptyBody)?
                    .chars()
                    .as_str()[..32]
                    .to_string(),
            ),
            source: match item.source {
                Some(source) => Some(source.url),
                None => None,
            },
            description: item.description,
            created: item.pub_date.and_then(|date| date.parse().ok()),
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
        write!(f, "{} {}", status, self.name)
    }
}

impl From<rss::Enclosure> for MediaPayload {
    fn from(value: rss::Enclosure) -> Self {
        Self {
            url: value.url,
            mime_type: value.mime_type,
            downloaded: false,
        }
    }
}

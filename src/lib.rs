#[derive(Debug)]
pub struct Article {
    pub source: Option<String>,
    pub description: Option<String>,
    pub body: ArticleBody,
    pub created: Option<String>, // should be date, eventually
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
            source: match item.source {
                Some(source) => Some(source.url),
                None => None,
            },
            description: item.description,
            created: item.pub_date,
            viewed: Progress::None,
            body,
        })
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

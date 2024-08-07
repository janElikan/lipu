use color_eyre::eyre;
use std::fs;
use std::io;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let file = fs::File::open("feed.xml").unwrap();
    let buffer = io::BufReader::new(file);
    let feed = rss::Channel::read_from(buffer).unwrap();

    let articles: Vec<_> = feed.items.into_iter().map(Article::try_from).collect();

    dbg!(articles);

    Ok(())
}

// idk what to call it, FeedItem is long
#[derive(Debug)]
struct Article {
    source: Option<String>,
    description: Option<String>,
    body: ArticleBody,
    created: Option<String>, // should be date, eventually
    viewed: Progress,
}

#[derive(Debug)]
enum ArticleBody {
    Text(String),
    Video(MediaPayload),
    Audio(MediaPayload),
}

#[derive(Debug)]
enum Progress {
    None,
    UntilLine(usize),
    UntilSecond(usize),
    Fully,
}

#[derive(Debug)]
struct MediaPayload {
    url: String,
    mime_type: String,
    downloaded: bool,
}

#[derive(Debug)]
enum ArticleCreationError {
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

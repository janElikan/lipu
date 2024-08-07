use color_eyre::eyre;
use std::fs;
use std::io;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let file = fs::File::open("feed.xml").unwrap();
    let buffer = io::BufReader::new(file);
    let feed = rss::Channel::read_from(buffer).unwrap();

    let articles: Vec<_> = feed.items.into_iter().map(Article::from).collect();

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

impl From<rss::Item> for Article {
    fn from(item: rss::Item) -> Self {
        let body = match item.enclosure {
            Some(media) => match media.mime_type.split_once('/').unwrap().0 {
                "audio" => ArticleBody::Audio(media.into()),
                "video" => ArticleBody::Video(media.into()),
                _ => todo!(),
            },
            None => {
                let content = item.content.unwrap_or(
                    item.description
                        .clone()
                        .expect("found an item without any body"),
                );
                ArticleBody::Text(content)
            }
        };

        Self {
            source: match item.source {
                Some(source) => Some(source.url),
                None => None,
            },
            description: item.description,
            created: item.pub_date,
            viewed: Progress::None,
            body,
        }
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

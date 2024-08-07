use color_eyre::eyre;
use std::fs;
use std::io;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let file = fs::File::open("feed.xml").unwrap();
    let buffer = io::BufReader::new(file);
    let feed = rss::Channel::read_from(buffer).unwrap();

    let articles: Vec<_> = feed
        .items
        .into_iter()
        .map(lipu::Article::try_from)
        .collect();

    dbg!(articles);

    Ok(())
}

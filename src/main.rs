use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let articles = fetch("http://www.0atman.com/feed.xml").await?;

    dbg!(articles);

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

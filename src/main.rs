use std::fs;

use color_eyre::eyre::Result;
use lipu::{
    core::{Body, ViewingProgress},
    App,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new()
        .add_feed("https://www.0atman.com/feed.xml")
        .add_feed("https://www.spreaker.com/show/4488937/episodes/feed") // LT
        .add_feed("https://www.spreaker.com/show/6029902/episodes/feed") // TPC
        .add_feed("https://www.youtube.com/feeds/videos.xml?channel_id=UCUMwY9iS8oMyWDYIe6_RmoA"); // NB

    loop {
        let mut articles = app.fetch().await;

        let selected =
            inquire::Select::new("What do you want to view?", articles.clone()).prompt()?;

        articles
            .iter_mut()
            .find(|candidate| candidate.id == selected.id)
            .expect("Error in the inquire crate")
            .viewed = ViewingProgress::Fully;

        match &selected.body {
            Body::Html(text) => println!("#{}\n{}", selected.name, text),
            Body::Audio(payload) => {
                if !confirm_download()? {
                    continue;
                }

                let path = format!("/tmp/{}", selected.name);
                let stream = reqwest::get(&payload.url).await?.bytes().await?;

                fs::write(path, stream)?;
            }
            Body::YouTubeLink(link) => println!("\n\n{link}\n\n"),
            Body::Video(_payload) => {
                todo!()
            }
        }

        app = app.copy_statuses(articles);
    }
}

fn confirm_download() -> Result<bool> {
    Ok(inquire::Confirm::new("Download?")
        .with_default(true)
        .prompt()?)
}

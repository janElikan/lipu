use std::fs;

use color_eyre::eyre::Result;

use lipu::{Lipu, LipuInterface}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = Lipu::new();

    app.add_feed("https://www.0atman.com/feed.xml")
    app.add_feed("https://www.spreaker.com/show/4488937/episodes/feed") // LT
    app.add_feed("https://www.spreaker.com/show/6029902/episodes/feed") // TPC
    app.add_feed("https://www.youtube.com/feeds/videos.xml?channel_id=UCUMwY9iS8oMyWDYIe6_RmoA"); // NB

    loop {
        app.refresh().await?;

        let selected =
            inquire::Select::new("What do you want to view?", app.list()).prompt()?;

        match &selected.body {
            _ => todo!(),
        }
    }
}

fn confirm_download() -> Result<bool> {
    Ok(inquire::Confirm::new("Download?")
        .with_default(true)
        .prompt()?)
}

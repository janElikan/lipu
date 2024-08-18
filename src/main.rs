use color_eyre::eyre::Result;

use lipu::Lipu;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = Lipu::new();

    app.add_feed("https://www.0atman.com/feed.xml".to_string());
    app.add_feed("https://www.spreaker.com/show/4488937/episodes/feed".to_string()); // LT
    app.add_feed("https://www.spreaker.com/show/6029902/episodes/feed".to_string()); // TPC
    app.add_feed(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCUMwY9iS8oMyWDYIe6_RmoA".to_string(),
    ); // NB

    loop {
        app.refresh().await.unwrap();

        let selected = inquire::Select::new(
            "What do you want to view?",
            app.list()
                .into_iter()
                .map(|item| format!("{}", item.id))
                .collect(),
        )
        .prompt()?;

        dbg!(app.load(&selected));
    }
}

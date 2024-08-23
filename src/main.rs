use color_eyre::eyre::Result;

use lipu::Lipu;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = Lipu::new("data".into());

    app.add_feed("https://xeiaso.net/xecast.rss".to_string());
    app.add_feed("https://www.spreaker.com/show/4488937/episodes/feed".to_string()); // LT

    app.refresh().await.unwrap();
    let list = app.list();
    let item = list.first().unwrap();
    println!("just started");
    dbg!(app.load(&item.id));
    app.download_item(&item.id).await.unwrap();

    println!("downloaded");
    dbg!(app.load(&item.id));

    app.write_to_disk().await.unwrap();

    Ok(())
}

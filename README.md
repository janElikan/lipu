The logic part of [lipu](https://janelikan.github.io/lipu), rss reader back-end.

## Usage

```rust
use lipu::App;

// App is just a builder to get Vec<Article>
let app: App = App::new().add_feed("https://fasterthanli.me/index.xml");
let articles: Vec<Article> = app.fetch().await;
```

A [CLI app exmaple](https://github.com/janElikan/lipu/blob/main/src/main.rs).

## Types

```rust
pub struct Article {
    pub id: String,
    pub name: String,
    pub source: Option<String>,
    pub description: Option<String>,
    pub body: ArticleBody,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub viewed: Progress,
}

pub enum ArticleBody {
    Text(String),
    Video(MediaPayload),
    Audio(MediaPayload),
    YouTubeLink(String),
}

pub enum Progress {
    None,
    UntilLine(usize),
    UntilSecond(usize),
    Fully,
}

pub struct MediaPayload {
    pub url: String,
    pub mime_type: String,
    pub downloaded: bool,
}
```

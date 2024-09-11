use leptos::{component, view, For, IntoView, ReadSignal, WriteSignal};

#[component]
pub fn FeedList(
    feeds: ReadSignal<Vec<lipu::Feed>>,
    set_feeds: WriteSignal<Vec<lipu::Feed>>,
) -> impl IntoView {
    view! {
        <div>
            <h2>"Feeds"</h2>
            <ul>
                <For each=feeds key=|item| item.url.clone() let:child>
                    <Feed feed={child} />
                </For>
            </ul>
        </div>
    }
}

#[component]
pub fn Feed(feed: lipu::Feed) -> impl IntoView {
    view! {
        <li>
            <h2>"feed title"</h2>
            <a href="#">{feed.url}</a>
        </li>
    }
}

#[component]
pub fn ItemList() -> impl IntoView {
    view! {
        <div>
            <h2>"Items"</h2>
            <input type="text" placeholder="search" />
            <ul>
                <Description />
            </ul>
        </div>
    }
}

pub fn Description() -> impl IntoView {
    view! {
        <li>
            <h2>"item title"</h2>
            <p>
                "title"
                <br />
                "posted YYYY-MM-DDTHH:MM:SSZ"
                <br />
                "read x%"
            </p>
        </li>
    }
}

#[component]
pub fn Reader() -> impl IntoView {
    view! {
        <p>"TODO"</p>
    }
}

#[component]
pub fn Player() -> impl IntoView {
    view! {
        <p>"TODO"</p>
    }
}

use leptos::{component, view, IntoView};

#[component]
pub fn FeedList() -> impl IntoView {
    view! {
        <div>
            <h2>"Feeds"</h2>
            <ul>
                <Feed />
            </ul>
        </div>
    }
}

#[component]
pub fn Feed() -> impl IntoView {
    view! {
        <li>
            <h2>"feed title"</h2>
            <a href="#">"url"</a>
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

use leptos::{component, create_signal, view, IntoView, SignalUpdate};

mod ui;
use ui::*;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (feeds, set_feeds) = create_signal(Vec::new());

    view! {
        <div class="grid container">
            <FeedList feeds={feeds} set_feeds={set_feeds} />
            <ItemList />
            <main>
                <Reader />
                <Player />
            </main>
        </div>
    }
}

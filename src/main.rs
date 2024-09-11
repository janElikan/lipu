use leptos::{component, create_signal, view, IntoView, SignalUpdate};

mod ui;
use ui::*;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="grid container">
            <FeedList />
            <ItemList />
            <main>
                <Reader />
                <Player />
            </main>
        </div>
    }
}

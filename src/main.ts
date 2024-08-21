import { backend, Metadata } from "./backend";
import { items } from "./items";

export const utils = {
    wrapInput: (backendFn: (data: string) => Promise<any>) => {
        return (event: Event) => backendFn((event.target as any)?.value || "");
    },
    wrapInputAndThen: <T>(
        backendFn: (data: string) => Promise<T>,
        then: (response: T) => any
    ) => {
        return (event: Event) =>
            backendFn((event.target as any)?.value || "").then(then);
    },
};

export function renderDescription(metadata: Metadata) {
    const element = document.createElement("li");
    element.innerText = JSON.stringify(metadata, null, 4);
    element.className = "description";

    return element;
}

async function init() {
    // feeds.init() todo
    items.init();
    await items.refresh();
}

window.addEventListener("DOMContentLoaded", async () => {
    await backend.addFeed("https://www.0atman.com/feed.xml");
    await backend.addFeed("https://xeiaso.net/xecast.rss");
    await backend.addFeed(
        "https://www.spreaker.com/show/4488937/episodes/feed"
    ); // LT
    await backend.addFeed(
        "https://www.spreaker.com/show/6029902/episodes/feed"
    ); // TPC
    await backend.addFeed(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCUMwY9iS8oMyWDYIe6_RmoA"
    ); // NB

    await init();
});

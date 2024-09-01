import { readFile } from "@tauri-apps/plugin-fs";
import { backend, Metadata, processResource, RawResource } from "./backend";
import { items } from "./items";
import { reader } from "./reader";
import { player } from "./player";
import { generalizedType } from "./files";

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

export function renderDescription(
    metadata: Metadata,
    onOpen?: (id: string) => void
) {
    const box = document.createElement("li");
    box.className = "description";

    const text = document.createElement("pre");
    text.innerText = JSON.stringify(metadata, null, 4);
    box.appendChild(text);

    if (onOpen) {
        const open = document.createElement("button");
        open.innerText = "open";
        open.addEventListener("click", () => onOpen(metadata.id));
        box.appendChild(open);
    }

    return box;
}

export async function handleOpen(id: string) {
    const item = await backend.load(id);
    const body = processResource(item.body);
    const bodyType = generalizedType(body);

    reader.open(item.metadata, body);

    if (bodyType === "audio" || bodyType === "video") {
        await player.open(item.metadata, body);
    }
}

async function init() {
    await backend.fetchAssetPath();
    // feeds.init() todo
    items.init();
}

window.addEventListener("DOMContentLoaded", async () => {
    await backend.addFeed("https://www.0atman.com/feed.xml").catch(console.log);
    await backend.addFeed("https://fasterthanli.me/index.xml");
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

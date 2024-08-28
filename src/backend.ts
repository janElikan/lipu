import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { appDataDir, join } from "@tauri-apps/api/path";

export const backend = {
    addFeed(url: string) {
        return invoke("add_feed", { url });
    },
    addMastodonFeed(instance: string, user: string) {
        return invoke("add_mastodon_feed", { instance, user });
    },
    addYoutubeChannel(channelId: string) {
        return invoke("add_youtube_channel", { channelId });
    },
    refresh() {
        return invoke("refresh");
    },
    removeFeed(url: string) {
        return invoke("remove_feed", { url });
    },

    list() {
        return invoke<Metadata[]>("list");
    },
    search(query: string) {
        if (!query) {
            return backend.list();
        }

        return invoke<Metadata[]>("search", { query });
    },
    withTag(tag: string) {
        return invoke<Metadata[]>("with_tag", { tag });
    },

    addTag(itemId: string, tag: string) {
        return invoke("add_tag", { itemId, tag });
    },
    removeTag(itemId: string, tag: string) {
        return invoke("remove_tag", { itemId, tag });
    },
    dropTag(tag: string) {
        return invoke("drop_tag", { tag });
    },

    load(itemId: string) {
        return invoke<Item>("load", { itemId });
    },
    setViewingProgress(itemId: string, progress: ViewingProgress) {
        return invoke("set_viewing_progress", { itemId, progress });
    },
    downloadItem(itemId: string) {
        return invoke("download_item", { itemId });
    },

    assetPath: "",
    fetchAssetPath: async() => {
        backend.assetPath = await join(await appDataDir(), "lipu")
    },
};

export type Item = {
    metadata: Metadata;
    body: RawResource;
};

export type Metadata = {
    id: string;

    name: string;
    tags: string[];

    feed_url: string;
    link?: string;
    author?: string;
    description?: string;

    thubmnail?: RawResource;

    created?: string;
    updated?: string;

    viewed: ViewingProgress;
};

type RawDownloadLinkResource = {
          DownloadLink: {
              mime_type?: string;
              url: string;
          };
      }

type RawFileResource =  {
          File: {
              mime_type?: string;
              path: string;
          };
      }

export type RawResource =
    | RawDownloadLinkResource
    | RawFileResource
    | "Missing";

export type Resource = {
    type: "downloadLink" | "file" | "void",
    url: string | null;
    mimeType: string | null;
    local: boolean;
};

export type ViewingProgress =
    | "Zero"
    | { UntilParagraph: number }
    | { UntilSecond: number }
    | "Fully";

export function processResource(resource: RawResource): Resource {
    const type = determineResourceType(resource);

    if (type === "downloadLink") {
        const {url, mime_type} = (resource as RawDownloadLinkResource).DownloadLink;

        return {type, url: wrapUrl(url), mimeType: mime_type || null, local: false};
    } else if (type === "file") {
        const {path, mime_type} = (resource as RawFileResource).File;

        return {type, url: wrapUrl(path), mimeType: mime_type || null, local: true};
    } else {
        return { type, url: null, mimeType: null, local: true };
    }
}

function determineResourceType(resource: RawResource) {
    if (typeof resource !== "object") {
        return "void";
    }

    let key = Object.keys(resource)[0];

    if (key === "DownloadLink") {
        return "downloadLink";
    } else if (key === "File") {
        return "file";
    } else {
        return "void";
    }
}

function wrapUrl(filename: string) {
    return convertFileSrc(backend.assetPath + "/" + filename);
}

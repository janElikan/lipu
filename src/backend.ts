import { invoke } from "@tauri-apps/api/core";

export const backend = {
    addFeed(url: string) {
        return invoke("add_feed", { url });
    },
    addMastodonFeed(instance: string, user: string) {
        return invoke("add_mastodon_feed", { instance, user });
    },
    addYoutubeChannel(channelId: string) {
        return invoke("add_youtube_channel", { channel_id: channelId });
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
            return backend.list()
        }

        return invoke<Metadata[]>("search", { query });
    },
    withTag(tag: string) {
        return invoke<Metadata[]>("with_tag", { tag });
    },

    addTag(itemId: string, tag: string) {
        return invoke("add_tag", { item_id: itemId, tag });
    },
    removeTag(itemId: string, tag: string) {
        return invoke("remove_tag", { item_id: itemId, tag });
    },
    dropTag(tag: string) {
        return invoke("drop_tag", { tag });
    },

    load(itemId: string) {
        return invoke<Item>("load", { item_id: itemId });
    },
    setViewingProgress(itemId: string, progress: ViewingProgress) {
        return invoke("set_viewing_progress", { item_id: itemId, progress });
    },
    downloadItem(itemId: string) {
        return invoke("download_item", { item_id: itemId });
    },
};

export type Item = {
    metadata: Metadata;
    body: Resource;
};

export type Metadata = {
    id: string;

    name: string;
    tags: string[];

    feed_url: string;
    link?: string;
    author?: string;
    description?: string;

    thubmnail?: Resource;

    created?: string;
    updated?: string;

    viewed: ViewingProgress;
};

export type Resource =
    | {
          DownloadLink: {
              mime_type?: string;
              url: string;
          };
      }
    | {
          File: {
              mime_type?: string;
              path: string;
          };
      }
    | "Missing";

export type ViewingProgress =
    | "Zero"
    | { UntilParagraph: number }
    | { UntilSecond: number }
    | "Fully";

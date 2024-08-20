import { backend } from "./backend";

const utils = {
  wrapInput: (callback: (data: string) => Promise<any>) => {
    return (event: Event) => callback((event.target as any)?.value || "");
  },
};

window.addEventListener("DOMContentLoaded", () => {
  backend.addFeed("https://www.0atman.com/feed.xml");
  backend.addFeed("https://xeiaso.net/xecast.rss");
  backend.addFeed("https://www.spreaker.com/show/4488937/episodes/feed"); // LT
  backend.addFeed("https://www.spreaker.com/show/6029902/episodes/feed"); // TPC
  backend.addFeed("https://www.youtube.com/feeds/videos.xml?channel_id=UCUMwY9iS8oMyWDYIe6_RmoA"); // NB

  const elements = {
    list: {
      search: document.querySelector('#list-search'),
      refresh: document.querySelector('#list-refresh'),
      list: document.querySelector('#list-list'),
    },
  };

  elements.list.search?.addEventListener("onchange", utils.wrapInput(backend.search))
});

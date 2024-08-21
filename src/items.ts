import { backend } from "./backend";
import { renderDescription, utils } from "./main";

const elements = {
    refresh: document.querySelector("#items-refresh") as HTMLButtonElement,
    search: document.querySelector("#items-search") as HTMLInputElement,
    list: document.querySelector("#items-list") as HTMLUListElement,
};

export const items = {
    init: () => {
        elements.refresh.addEventListener("click", items.refresh);
        elements.search.addEventListener(
            "input",
            utils.wrapInput(items.search)
        );
    },
    refresh: async () => {
        elements.list.innerHTML = "<p>Loading...</p>";
        await backend.refresh();
        await items.search(elements.search.value);
    },
    search: async (query: string) => {
        elements.list.innerHTML = "";

        const items = query
            ? await backend.search(query)
            : await backend.list();
        items
            .map(renderDescription)
            .forEach((leaf) => elements.list.appendChild(leaf));
    },
};

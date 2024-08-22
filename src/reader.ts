import { Item } from "./backend";
import { renderDescription } from "./main";

const elements = {
    description: document.querySelector(
        "#reader-description"
    ) as HTMLDivElement,
    content: document.querySelector("#reader-content") as HTMLDivElement,
};

export const reader = {
    open: (item: Item) => {
        elements.description.innerHTML = "";
        elements.description.appendChild(renderDescription(item.metadata));

        // todo handle Body
        elements.content.innerHTML =
            item.metadata.description || "(empty body)";
    },
};

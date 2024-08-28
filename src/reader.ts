import { backend, Metadata, processResource, RawResource, Resource } from "./backend";
import { generalizedType } from "./files";
import { handleOpen, renderDescription } from "./main";

const elements = {
    description: document.querySelector(
        "#reader-description"
    ) as HTMLDivElement,
    content: document.querySelector("#reader-content") as HTMLDivElement,
};

function render(html: string) {
    elements.content.innerHTML = html;
}

function renderThumbnail(thumbnail?: RawResource) {
    if (!thumbnail) {
        return
    }

    const {url, local} = processResource(thumbnail);

    if (!local) {
        return;
    }

    const element = document.createElement("img");
    element.src = url || "";
    element.alt = "alts are not yet supported";

    elements.content.appendChild(element);
}

export const reader = {
    open: (metadata: Metadata, body: Resource) => {
        elements.description.innerHTML = "";
        elements.content.innerHTML = "";
        elements.description.appendChild(renderDescription(metadata));

        if (!body.local) {
            const download = document.createElement("button");
            download.innerText = "Download";
            download.addEventListener("click", async () => {
                await backend.downloadItem(metadata.id).catch(console.log);

                alert(`Downloaded #${metadata.id}`);
                handleOpen(metadata.id);
            })
            elements.content.appendChild(download);
            return;
        }

        if (!body.url) {
            render(metadata.description || "<p>(empty body)</p>");
            return
        }

        const contentType = generalizedType(body);
        if (contentType === "void") {
            render("<p>(empty body)</p>");
        } else if (contentType === "text") {
            // idk what I'm doing, but at least it's not downloading the thing in JS
            // should probably read about iframes more on MDN
            const iframe = document.createElement("iframe");
            iframe.src = body.url;
            elements.content.appendChild(iframe);
        } else if (contentType === "audio") {
            renderThumbnail(metadata.thubmnail);

            const description = document.createElement("p");
            description.innerHTML = metadata.description || "(no description provided)";
            elements.content.append(description);
        } else {
            render("<p>?</p>")
        }
    },
};

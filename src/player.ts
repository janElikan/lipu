import { backend, Metadata, Resource } from "./backend";

const elements = {
    root: document.querySelector("#player") as HTMLDivElement,
    audio: document.querySelector("#player-audio") as HTMLAudioElement,
};

export const player = {
    async open(_metadata: Metadata, body: Resource) {
      elements.root.classList.remove("hidden");
      elements.audio.src = await backend.loadFile(body);
    }
}

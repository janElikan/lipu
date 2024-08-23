import { Resource } from "./backend";

export function generalizedType({mimeType}: Resource) {
  if (!mimeType) {
    return "void";
  }

  const [kind, _] = mimeType.split("/");

  if (kind === "audio" || kind === "video" || kind === "text") {
    return kind;
  } else {
    return "unknown";
  }
}

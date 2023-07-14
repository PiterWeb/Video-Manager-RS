import { writable } from "svelte/store";

export const starredVideos = writable<string[]>([]);

import { writable } from "svelte/store";

export const selectedVideo = writable<string | null>(null);
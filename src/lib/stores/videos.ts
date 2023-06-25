import { writable } from "svelte/store";

export const videos = writable<string[]>([]);
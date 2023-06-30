import { writable } from "svelte/store";

export const videosSelectedForActions = writable<string[]>([]);
import { writable } from "svelte/store";

import type Filter from "$lib/types/Filter";

export const activeFilters = writable<Filter[]>([]);
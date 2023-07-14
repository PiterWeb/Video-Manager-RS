import { invoke } from "@tauri-apps/api/tauri";
import { starredVideos } from "$lib/stores/starredVideos";

export const addStarred = async (path: string) => {

    invoke('set_favorite_video', { path }).then(() => {
        starredVideos.update((value) => {
            return [...value, path];
        })
    }).catch((e) => console.error(e));
}
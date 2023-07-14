import { invoke } from "@tauri-apps/api/tauri";
import { starredVideos } from "$lib/stores/starredVideos";

export const removeStarred = async (path: string) => {

    invoke('unset_favorite_video', { path }).then(() => {

        starredVideos.update((stVideos) => stVideos.filter((v) => v != path));

    }).catch((e) => console.error(e));

}
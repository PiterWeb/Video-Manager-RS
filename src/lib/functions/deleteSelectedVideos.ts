import { invoke } from "@tauri-apps/api"
import { videos } from "$lib/stores/videos";

export const deleteSelectedVideos = async (paths: string[]) => {

    invoke('delete_selected_videos', { paths }).then(() => {

        paths.forEach((path) => {
            videos.update((value) => {
                const index = value.indexOf(path);
                if (index === -1) return value;
                return value.toSpliced(index, 1);
            })
        })

    }).catch((e) => console.error(e));

}

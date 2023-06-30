import { invoke } from "@tauri-apps/api/tauri";
import { DBInsertVideoError } from "$lib/types/DBError";
import { videos } from "$lib/stores/videos";

const extensions = ['mp4', 'mkv', 'avi']

const updateVideos = (path: string) => {
    videos.update((value) => {
        if (value.includes(path)) return value;
        value.push(path)
        return value;
    });
}

export const saveFiles = async (files: string | string[]) => {
    if (files instanceof Array) {
        files.forEach(async (path) => {
            
            if (!extensions.some((ext) => path.endsWith(ext))) throw new Error('Invalid file extension');

            const response = await invoke<string>('insert_video', {
                path
            });

            if (response.includes(DBInsertVideoError)) throw new Error("Error inserting video on DB (maybe it's on your manager already)");

            updateVideos(path);
        });

        return;
    }

    if (!extensions.some((ext) => files.endsWith(ext))) throw new Error('Invalid file extension');


    const response = await invoke<string>('insert_video', {
        path: files
    });

    if (response.includes(DBInsertVideoError)) throw new Error(response);

    updateVideos(files);
};
import { invoke } from "@tauri-apps/api/tauri";

export const getStarredsOnInit = async () => {

        const favorites = await invoke('get_favorite_videos') as string[];

        return favorites;
}
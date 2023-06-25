import { invoke} from '@tauri-apps/api/tauri';

export const getVideos = async () => {
    const response = await invoke<string[]>('get_videos');

    return response;
};
import { open } from '@tauri-apps/api/dialog';
import { saveFiles } from '$lib/functions/saveFiles';

export const selectFiles = async () => {
    const result = await open({
        defaultPath: '.',
        multiple: true,
        directory: false,
        filters: [{ name: 'Videos', extensions: ['mp4', 'mkv', 'avi'] }]
    });

    if (result === null) return;

    saveFiles(result).catch((e) => console.error(e));

    // Save path on DB and update the list
};
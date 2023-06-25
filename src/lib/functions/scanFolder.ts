import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';

export const scanFolder = async () => {
    const result = await open({
        defaultPath: '.',
        multiple: false,
        directory: true
    });

    if (result === null) return;

    return invoke<void>('scan_folder', {
        path: result as string
    })
};
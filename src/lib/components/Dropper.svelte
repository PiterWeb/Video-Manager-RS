<script lang="ts">
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import { appWindow } from '@tauri-apps/api/window';
	import { readDir } from '@tauri-apps/api/fs';
	import { saveFiles } from '$lib/functions/saveFiles';
	import { selectFiles as selectFilesPromise } from '$lib/functions/selectFiles';

	import { i } from '@inlang/sdk-js';

	let fileDropState: 'hover' | 'drop' | 'cancel' = 'cancel';
	let fileDropPaths: string[] = [];

	const successModal = {
		title: 'Operation finished!',
		message: 'File/s successfully added to the system'
	};

	const errorModal = {
		title: 'Operation failed!',
		message: 'File/s could not be added to the system'
	};

	let modal: HTMLDialogElement;
	let modalTitle: string = successModal.title;
	let modalMessage: string = successModal.message;

	const fileMessage = {
		hover: 'Files to be saved',
		drop: 'Saving files',
		cancel: 'Drop your files here'
	};

	type FileDropEvent = {
		type: 'hover' | 'drop' | 'cancel';
		paths: string[];
	};

	let unlistenFileDrop: Promise<UnlistenFn>;

	function selectFiles() {
		selectFilesPromise()
			.then(() => {
				fileDropState = 'drop';
			})
			.catch(({ message: errorMessage }: { message?: string }) => {
				modalTitle = errorModal.title;
				modalMessage = errorMessage || errorModal.message;
				modal.showModal();
			});
	}

	onMount(() => {
		unlistenFileDrop = appWindow.onFileDropEvent((event) => {
			fileDropState = event.payload.type;
			fileDropPaths = fileDropState !== 'cancel' ? (event.payload as FileDropEvent).paths : [];
			if (fileDropState === 'drop') {
				fileDropPaths.forEach(async (path) => {
					try {
						const dir = await readDir(path);

						console.table(dir);

						saveFiles(dir.map((file) => file.path));
					} catch (e) {
						saveFiles(path).catch(({ message: errorMessage }: { message?: string }) => {
							modalTitle = errorModal.title;
							modalMessage = errorMessage || errorModal.message;
						});
					}
				});

				setTimeout(() => {
					modal.showModal();
					fileDropState = 'cancel';
					fileDropPaths = [];
				}, 1000);
			}
		});
	});

	onDestroy(async () => {
		(await unlistenFileDrop)();
	});
</script>

<dialog bind:this={modal} class="modal">
	<form method="dialog" class="modal-box">
		<h3 class="font-bold text-lg">{modalTitle}</h3>
		<p class="py-4">{modalMessage}</p>
		<div class="modal-action">
			<!-- if there is a button in form, it will close the modal -->
			<button class="btn btn-primary">{i("close")}</button>
		</div>
	</form>
</dialog>

<section
	class="flex flex-col basis-2/3 items-center text-center justify-center w-full h-[calc(100vh-12rem)] p-12 space-y-4 border-2 border-dashed border-primary rounded-lg transition-colors"
	class:border-neutral={fileDropState === 'drop'}
	class:border-secondary={fileDropState === 'hover'}
	on:dragover={(event) => {
		event.preventDefault();
		fileDropState = 'hover';
	}}
>
	{#if fileDropState === 'drop'}
		<span class="loading loading-dots loading-lg transition-all" />
	{/if}
	<p
		class="text-2xl select-none hover:cursor-pointer"
		on:click={selectFiles}
		on:keydown={selectFiles}
	>
		{fileMessage[fileDropState]}
	</p>
	<div class="text-sm text-base-content select-none grid grid-rows-2 gap-4">
		{#each fileDropPaths as path, index}
			{#if index < 5}
				<div class="badge badge-neutral">{path}</div>
			{:else if index === 5}
				<div class="badge badge-neutral">+{fileDropPaths.length - 5} {i("more").toLowerCase()}</div>
			{/if}
		{/each}
	</div>
</section>

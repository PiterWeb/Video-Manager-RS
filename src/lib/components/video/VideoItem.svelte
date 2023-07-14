<script lang="ts">
	export let video: string;

	import { onMount } from 'svelte';
	import { exists } from '@tauri-apps/api/fs';

	import { selectedVideo } from '$lib/stores/selectedVideo';
	import { deleteSelectedVideos } from '$lib/functions/deleteSelectedVideos';

	import SelectButton from '$lib/components/video/SelectForActionsButton.svelte';
	import FavouriteButton from '$lib/components/video/FavouriteButton.svelte';

	function deleteVideo() {
		deleteSelectedVideos([video]);
	}

	function selectVideo() {
		selectedVideo.set(video);
	}

	onMount(async () => {
		const fileExists = await exists(video);

		if (!fileExists) {
			deleteVideo();
		}
	});
</script>

<li
	class="py-2 rounded-lg transition-opacity duration-500 cursor-pointer"
	class:bg-base-300={$selectedVideo === video}
	on:click={selectVideo}
	on:keydown={selectVideo}
>
	<span class="flex flex-row justify-start group transition-opacity duration-500">
		<SelectButton {video} />

		<div class="grid grid-cols-2 w-full items-center pl-2">
			{video.split('\\').at(-1)}

			<FavouriteButton {video} />
		</div>
	</span>
</li>

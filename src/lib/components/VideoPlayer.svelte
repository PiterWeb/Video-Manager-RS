<script lang="ts">
	import { selectedVideo } from '$lib/stores/selectedVideo';
	import { appWindow } from '@tauri-apps/api/window';
	import { tauri } from '@tauri-apps/api';

	import { i } from '@inlang/sdk-js';

	let showVideo = false;
	let fullscreen = false;
	let player: any;
	let videoPath = !$selectedVideo ? '' : tauri.convertFileSrc($selectedVideo);

	function toggleFullscreen() {
		console.log('toggleFullscreen');
		fullscreen = !fullscreen;

		appWindow.setFullscreen(fullscreen);
	}

	async function initializePLayer() {
		if (!$selectedVideo) return;

		videoPath = tauri.convertFileSrc($selectedVideo);

		// @ts-ignore
		player = new window.Playerjs({
			id: 'player',
			file: videoPath,
			autoplay: false,
		});
	}

	async function reloadPlayer() {
		if (player) player.api('file', videoPath, '*');
		else initializePLayer();
	}

	$: if ($selectedVideo) {
		showVideo = true;
		videoPath = tauri.convertFileSrc($selectedVideo);
		reloadPlayer();
	}
</script>

<svelte:head>
	<script defer src="/playerjs.js" type="text/javascript" on:load={initializePLayer}></script>
</svelte:head>
<div id="player" class="min-w-[20rem] max-w-full" on:fullscreenchange={toggleFullscreen} />
{#if !showVideo}
	<div class="flex flex-col items-center justify-center h-full">
		<p class="text-2xl font-bold">{i("video_player.no_video")}</p>
		<p class="text-base">{i("video_player.no_video_description")}</p>
	</div>
{/if}

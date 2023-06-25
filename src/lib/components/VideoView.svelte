<script lang="ts">
	import { selectedVideo } from '$lib/stores/selectedVideo';
	import { appWindow } from '@tauri-apps/api/window';
	import { tauri } from '@tauri-apps/api';

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

		let videoPath = tauri.convertFileSrc($selectedVideo);

		// @ts-ignore
		player = new window.Playerjs({
			id: 'player',
			file: videoPath,
			autoplay: false
		});
	}

	async function reloadPlayer() {
		if (player) player.api('file', videoPath, '*');
		else initializePLayer();
	}

	$: if ($selectedVideo) {
		videoPath = tauri.convertFileSrc($selectedVideo);
		reloadPlayer();
	}
</script>

<svelte:head>
	<script defer src="/playerjs.js" type="text/javascript" on:load={initializePLayer}></script>
</svelte:head>

<!-- Video player -->
<section class="basis-1/3" id="player" on:fullscreenchange={toggleFullscreen} />
<!-- Video player -->

<script lang="ts">
	import VideoItem from '$lib/components/VideoItem.svelte';
	import { onMount } from 'svelte';

	import { getVideos } from '$lib/functions/getVideos';
	import { deleteSelectedVideos } from '$lib/functions/deleteSelectedVideos';

	import { drawerStore } from '$lib/stores/drawer';
	import { videos } from '$lib/stores/videos';
	import { videosSelectedForActions } from '$lib/stores/videosSelectedForActions';

	function cancelSelection() {
		videosSelectedForActions.set([]);
	}

	function deleteSelection() {
		videos.set($videos.filter((v) => !$videosSelectedForActions.includes(v)));
		deleteSelectedVideos($videosSelectedForActions);
		videosSelectedForActions.set([]);
	}

	onMount(async () => {
		videos.set(await getVideos());
	});
</script>

<section class="basis-2/3">
	<div class="join join-vertical w-full">
		<div>
			<p class="text-lg ml-4 mb-4 font-bold">Select a video to play</p>
		</div>
		<ul
			class="bg-base-300 p-4 transition-all duration-300 flex flex-row items-center gap-3"
			class:-translate-x-[200rem]={$videosSelectedForActions.length === 0}
		>
			<li class="px-2">
				{$videosSelectedForActions.length}

				{#if $videosSelectedForActions.length === $videos.length}
					<strong> (All) </strong>
				{/if}
				Selected
			</li>

			<li>
				<button class="btn btn-sm btn-error btn-outline" on:click={deleteSelection}>Delete </button>
			</li>
			<li>
				<button class="btn btn-sm btn-warning btn-outline" on:click={cancelSelection}>Cancel</button
				>
			</li>
		</ul>
	</div>

	<div />

	<ul
		class="menu bg-base-200 w-full max-h-[70vh] flex-nowrap -mt-16 transition-all duration-300 !overflow-y-auto"
		class:opacity-0={$drawerStore}
		class:translate-y-[6rem]={$videosSelectedForActions.length > 0}
	>
		{#if $videos.length > 0}
			{#each $videos as video}
				<VideoItem {video} />
			{/each}
		{:else}
			<li>
				<a class="flex flex-row gap-4 items-center" href="/add">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="w-4 h-4 ml-2"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M12 6v6m0 0v6m0-6h6m-6 0H6"
						/></svg
					>
					No videos found
				</a>
			</li>
		{/if}
	</ul>
</section>

<style>

ul {
		--scrollbarBG: #1e1e1e;
		--thumbBG: #888;
	}

	ul::-webkit-scrollbar {
		width: 10px;
		height: 10px;
	}

	ul::-webkit-scrollbar-track {
		background: #1e1e1e;
	}

	ul::-webkit-scrollbar-thumb {
		@apply bg-neutral rounded-full;
	}

</style>

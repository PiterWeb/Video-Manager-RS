<script lang="ts">
	import FilterButton from '$lib/components/video/FilterButton.svelte';
	import VideoItem from '$lib/components/video/VideoItem.svelte';

	import Filter from '$lib/types/Filter';

	import { i } from '@inlang/sdk-js';

	import { onMount } from 'svelte';

	import { getVideos } from '$lib/functions/getVideos';
	import { getStarredsOnInit } from '$lib/functions/getStarredsOnInit';

	import { drawerStore } from '$lib/stores/drawer';
	import { videos } from '$lib/stores/videos';
	import { activeFilters } from '$lib/stores/activeFilters';
	import { videosSelectedForActions } from '$lib/stores/videosSelectedForActions';
	import { starredVideos } from '$lib/stores/starredVideos';
	import SelectionMenu from './SelectionMenu.svelte';

	$: videosShowing = $videos.filter((v) => {
		if ($activeFilters.includes(Filter.OnlyStarred) && !$starredVideos.includes(v)) {
			return false;
		}

		if ($activeFilters.includes(Filter.OnlySelected) && !$videosSelectedForActions.includes(v)) {
			return false;
		}

		return true;
	});

	onMount(async () => {
		videos.set(await getVideos());
		starredVideos.set(await getStarredsOnInit());
	});
</script>

<section class="basis-2/3">
	<div class="join join-vertical w-full">
		<div class="mb-4 flex flex-row justify-between items-center">
			<p class="text-xl ml-4 font-bold">{i("video_herarchy.title")}</p>
			<FilterButton />
		</div>

		<SelectionMenu />
	</div>

	<ul
		class="menu bg-base-200 w-full max-h-[65vh] flex-nowrap -mt-16 transition-all duration-300 overflow-y-auto"
		class:hidden={$drawerStore}
		class:translate-y-[6rem]={$videosSelectedForActions.length > 0}
	>
		{#if videosShowing.length > 0}
			{#each videosShowing as video}
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
					{i("video_herarchy.no_videos")}
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

<script lang="ts">
	import VideoItem from '$lib/components/VideoItem.svelte';
	import { onMount } from 'svelte';

	import { getVideos } from '$lib/functions/getVideos';

	import { drawerStore } from '$lib/stores/drawer';
	import { videos } from '$lib/stores/videos';

	onMount(async () => {
		videos.set(await getVideos());
	});
</script>

<section class="basis-2/3">
	<!-- make the menu go to the back when drawer is open -->
	<ul
		class="menu bg-base-200 rounded-lg max-w-full w-full transition-all duration-300"
		class:hidden={$drawerStore}
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

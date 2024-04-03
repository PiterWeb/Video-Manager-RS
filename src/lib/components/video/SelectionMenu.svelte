<script lang="ts">
	import { videosSelectedForActions } from '$lib/stores/videosSelectedForActions';
	import { videos } from '$lib/stores/videos';
	import { deleteSelectedVideos } from '$lib/functions/deleteSelectedVideos';

	import {i} from '@inlang/sdk-js';

	function cancelSelection() {
		videosSelectedForActions.set([]);
	}

	function deleteSelection() {
		videos.set($videos.filter((v) => !$videosSelectedForActions.includes(v)));
		deleteSelectedVideos($videosSelectedForActions);
		videosSelectedForActions.set([]);
	}
</script>

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
		<button class="btn btn-sm btn-error btn-outline" on:click={deleteSelection}>{i("actions.delete")}</button>
	</li>

	<li>
		<button class="btn btn-sm btn-warning btn-outline" on:click={cancelSelection}>{i("actions.cancel")}</button>
	</li>
</ul>

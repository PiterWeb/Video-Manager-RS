<script lang="ts">
	export let video: string;

	import { selectedVideo } from '$lib/stores/selectedVideo';
	import { videosSelectedForActions } from '$lib/stores/videosSelectedForActions';

	import { deleteSelectedVideos } from '$lib/functions/deleteSelectedVideos';

	let selected = false;

	$: selected = $videosSelectedForActions.includes(video);

	function deleteVideo() {
		deleteSelectedVideos([video]);
	}

	function selectVideo() {
		selectedVideo.set(video);
	}

	function toggleCheckbox({ target }: Event) {
		if (!(target instanceof HTMLInputElement)) return;

		console.log(target.checked);

		if (target.checked) {
			$videosSelectedForActions.push(video);
			$videosSelectedForActions = $videosSelectedForActions;
			return;
		}

		$videosSelectedForActions = $videosSelectedForActions.filter((v) => v != video);
	}
</script>

<li
	class="py-2 rounded-lg transition-opacity duration-500 cursor-pointer"
	class:bg-base-300={$selectedVideo === video}
	on:click={selectVideo}
	on:keydown={selectVideo}
>
	<span class="flex flex-row justify-start group transition-opacity duration-500">
		<input
			type="checkbox"
			class="checkbox checkbox-primary"
			on:change={toggleCheckbox}
			checked={selected}
		/>
		<div class="flex flex-row items-center gap-2">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-4 h-4"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"
				/></svg
			>
		</div>

		<div class="grid grid-cols-2 gap-2 w-full items-center">
			{video.split('\\').at(-1)}

			<button
				on:click={deleteVideo}
				class="btn btn-error h-12 place-self-end opacity-0 group-hover:opacity-100 transition-opacity duration-300 self-center"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					class="inline-block w-4 h-4 stroke-current"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</button>
		</div>
	</span>
</li>

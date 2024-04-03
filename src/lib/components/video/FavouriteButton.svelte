<script lang="ts">
	export let video: string;

	import { starredVideos } from '$lib/stores/starredVideos';
	import { addStarred } from '$lib/functions/addStarred';
	import { removeStarred } from '$lib/functions/removeStarred';

	$: favorite = $starredVideos.includes(video);

	function toggleFavorite(e: MouseEvent) {
		e.stopPropagation();
		favorite = !favorite;
		const starButton = e.target as HTMLButtonElement;

		if (!favorite) {
			removeStarred(video);
			return;
		}

		addStarred(video);

		if (!starButton) return;

		starButton.classList.toggle('scale-150');
		starButton.classList.toggle('rotate-45');

		setTimeout(() => {
			starButton.classList.toggle('scale-150');
			starButton.classList.toggle('rotate-45');
		}, 200);
	}
</script>

<button
	class="mask mask-star-2 w-6 h-6 place-self-end self-center bg-slate-600 transition-all duration-200 ease-in-out"
	class:!bg-orange-400={favorite}
	on:click={toggleFavorite}
/>

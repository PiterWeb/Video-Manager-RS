<script lang="ts">
	import VideoPlayer from '$lib/components/VideoPlayer.svelte';

	import { selectedVideo } from '$lib/stores/selectedVideo';

	let name: string = '';

	function truncateText(text: string) {
		return text.split('\\').at(-1)!.length > 60
			? text.split('\\').at(-1)!.substring(0, 60) + '[...]'
			: text.split('\\').at(-1)!;
	}

	$: if ($selectedVideo) {
		name = truncateText($selectedVideo);
	}
</script>

<section class="basis-1/3 flex flex-col ">
	<VideoPlayer />
	<section>
		<p class="text-2xl font-bold">{name}</p>
		<p class="text-base" />
	</section>
</section>

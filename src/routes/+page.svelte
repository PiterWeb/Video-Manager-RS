<script lang="ts">
	import FileHerarchy from '$lib/components/FileHerarchy.svelte';
	import VideoView from '$lib/components/VideoView.svelte';

</script>

<div class="flex flex-row gap-12">
	<!-- File explorer (herarchy) -->
	<FileHerarchy />

	<VideoView />
</div>

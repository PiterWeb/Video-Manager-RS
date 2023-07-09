<script>
	import '../app.postcss';
	import Navbar from '$lib/components/Navbar.svelte';
</script>

<Navbar />

<main class="mx-12 my-12 h-full">
	<slot />
</main>

<style>

	:root {
		--scrollbarBG: #1e1e1e;
		--thumbBG: #888;
	}

	:root::-webkit-scrollbar {
		width: 10px;
		height: 10px;
	}

	:root::-webkit-scrollbar-track {
		background: #1e1e1e;
	}

	:root::-webkit-scrollbar-thumb {
		@apply bg-neutral rounded-full;
	}
</style>

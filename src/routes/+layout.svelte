<script>
	import '../app.postcss';
	import Navbar from '$lib/components/Navbar.svelte';
	import FloatingButton from '$lib/components/FloatingButton.svelte';
	import PageTransition from '$lib/components/PageTransition.svelte';

	import { page } from '$app/stores';
</script>

<svelte:head>
	<link rel="preload" href="/playerjs.js" as="script" />
</svelte:head>

<Navbar />

<PageTransition refresh={$page.route.id}>
	<slot />
</PageTransition>

<!-- <FloatingButton /> -->

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

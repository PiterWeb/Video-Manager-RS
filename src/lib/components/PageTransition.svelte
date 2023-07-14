<script lang="ts">
	import { slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	export let refresh: string | null = '';

	let innerWidth = 0;
	$: isSmall = innerWidth < 1025;
</script>

<svelte:window bind:innerWidth />

{#if isSmall}
	<main class="mx-12 my-12 h-full">
		<slot />
	</main>
{:else}
	{#key refresh}
		<main class="mx-12 my-12 h-full">
			<div in:slide={{ delay: 100, duration: 250, easing: quintOut, axis: 'x' }}>
				<slot />
			</div>
		</main>
	{/key}
{/if}
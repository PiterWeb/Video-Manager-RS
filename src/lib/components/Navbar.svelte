<script lang="ts">
	import { page } from '$app/stores';
	import { drawerStore } from '$lib/stores/drawer';

	import { i, languages, language, switchLanguage } from '@inlang/sdk-js';

	let path = $page.url.pathname;

	$: path = $page.url.pathname;

	const paths = {
		index: '/',
		add: '/add'
	};
</script>

<div class="drawer">
	<input id="my-drawer-3" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content flex flex-col">
		<!-- Navbar -->
		<nav class="w-full navbar bg-base-300">
			<div class="flex-none lg:hidden">
				<label
					for="my-drawer-3"
					class="btn btn-square btn-ghost"
					on:click={() => drawerStore.update((b) => !b)}
					on:keydown={() => drawerStore.update((b) => !b)}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						class="inline-block w-6 h-6 stroke-current"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 6h16M4 12h16M4 18h16"
						/></svg
					>
				</label>
			</div>
			<div class="flex-1 px-2 mx-2 font-bold">{i('app_name')}</div>
			<div class="flex-none hidden lg:block">
				<ul class="menu menu-horizontal gap-4">
					<!-- Navbar menu content here -->
					<li>
						<a href="/" class:active={path === paths.index}>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-5 w-5"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
								/></svg
							>
							{i('navbar.explore')}
						</a>
					</li>
					<li>
						<a href="/add" class:active={path === paths.add}>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								class="w-5 h-5 stroke-current"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
								/></svg
							>
							{i('navbar.add')}
						</a>
					</li>
					<li>
						{#each languages as lan}
							{#if language == lan}
								<li class="bg-primary text-white">{lan}</li>
							{:else}
								<li on:click={() => switchLanguage(lan)} on:keydown={() => switchLanguage(lan)}>{lan}</li>
							{/if}
						{/each}
					</li>
				</ul>
			</div>
		</nav>
		<slot />
	</div>
	<nav class="drawer-side">
		<label
			for="my-drawer-3"
			class="drawer-overlay"
			on:click={() => drawerStore.update((b) => !b)}
			on:keydown={() => drawerStore.update((b) => !b)}
		/>
		<ul class="menu p-4 w-80 h-full bg-base-200 gap-4">
			<!-- Sidebar content here -->
			<li>
				<a href="/" class:active={path === paths.index} class="normal-case text-xl btn"
					>{i('navbar.explore')}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5 mt-1"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
						/></svg
					></a
				>
			</li>
			<li>
				<a href="/add" class:active={path === paths.add} class="normal-case text-xl btn btn-ghost"
					>{i('navbar.add')}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						class="h-5 w-5 mt-1 stroke-current"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
						/></svg
					></a
				>
			</li>
		</ul>
	</nav>
</div>

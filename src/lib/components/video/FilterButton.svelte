<script lang="ts">
	import FilterSvg from '$lib/components/icons/FilterSVG.svelte';

	import Filter from '$lib/types/Filter';

	import { drawerStore } from '$lib/stores/drawer';
	import { activeFilters } from '$lib/stores/activeFilters';

	function toggleFilter(selectedFilter: Filter) {
		if ($activeFilters.includes(selectedFilter)) {
			activeFilters.set($activeFilters.filter((f) => f != selectedFilter));
			return;
		}

		activeFilters.update((filters) => [...filters, selectedFilter]);
	}
</script>

<div class="dropdown" class:hidden={$drawerStore}>
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<!-- svelte-ignore a11y-label-has-associated-control -->
	<div class="indicator mr-4">
		<span
			class="indicator-item badge badge-sm badge-secondary transition-opacity duration-200 ease-in-out animate-bounce"
			class:opacity-0={$activeFilters.length <= 0}>{$activeFilters.length}</span
		>
		<label tabindex="0" class="w-14 h-14 btn shadow-md group"><FilterSvg /></label>
	</div>
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<ul
		tabindex="0"
		class="dropdown-content z-[1] menu shadow bg-base-200 rounded-box py-5 px-2 border-base-100 border-2"
	>
		<li>
			<label class="cursor-pointer label">
				<span class="label-text">Only Starred</span>
				<input
					type="checkbox"
					class="toggle toggle-warning"
					on:change={() => toggleFilter(Filter.OnlyStarred)}
					checked={$activeFilters.includes(Filter.OnlyStarred)}
				/>
			</label>
		</li>
		<li>
			<label class="cursor-pointer label">
				<span class="label-text">Only Selected</span>
				<input
					type="checkbox"
					class="toggle toggle-primary"
					on:change={() => toggleFilter(Filter.OnlySelected)}
					checked={$activeFilters.includes(Filter.OnlySelected)}
				/>
			</label>
		</li>
	</ul>
</div>

<script lang="ts">
	export let video: string;

	$: selected = $videosSelectedForActions.includes(video);

	import { videosSelectedForActions } from '$lib/stores/videosSelectedForActions';

	function toggleCheckbox({ target }: Event) {
		if (!(target instanceof HTMLInputElement)) return;

		if (target.checked) {
			$videosSelectedForActions.push(video);
			$videosSelectedForActions = $videosSelectedForActions;
			return;
		}

		$videosSelectedForActions = $videosSelectedForActions.filter((v) => v != video);
	}
</script>

<input
	type="checkbox"
	class="checkbox checkbox-primary"
	on:change={toggleCheckbox}
	on:click={(e) => e.stopPropagation()}
	checked={selected}
/>

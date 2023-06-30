<script lang="ts">
	import { scanFolder } from '$lib/functions/scanFolder';

	const successModal = {
		title: 'Operation finished!',
		message: 'File/s successfully added to the system'
	};

	const errorModal = {
		title: 'Operation failed!',
		message: 'File/s could not be added to the system'
	};

	let modal: HTMLDialogElement;
	let modalTitle: string = successModal.title;
	let modalMessage: string = successModal.message;

	function scan() {
		scanFolder()
			.then(() => {
				modalTitle = successModal.title;
				modalMessage = successModal.message;
				modal.showModal();
			})
			.catch(({ message: errorMessage }: { message?: string }) => {
				modalTitle = errorModal.title;
				modalMessage = errorMessage || errorModal.message;
				modal.showModal();
			});
	}
</script>

<dialog bind:this={modal} class="modal">
	<form method="dialog" class="modal-box">
		<h3 class="font-bold text-lg">{modalTitle}</h3>
		<p class="py-4">{modalMessage}</p>
		<div class="modal-action">
			<!-- if there is a button in form, it will close the modal -->
			<button class="btn btn-primary">Close</button>
		</div>
	</form>
</dialog>

<section class="basis-1/3">
	<label for="scan" class="label text-xl normal-case font-bold">Scan directories</label>
	<button id="scan" class="btn btn-primary w-full" on:click={scan}>Select</button>
</section>

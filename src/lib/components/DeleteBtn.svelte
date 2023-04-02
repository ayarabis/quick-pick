<script lang="ts" context="module">
	import { store, type Item } from '$lib/app';
	import { modalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { emit } from '@tauri-apps/api/event';

	async function deleteResource(item: Item) {
		const confirm: ModalSettings = {
			type: 'confirm',
			title: 'Please Confirm',
			body: 'Are you sure you wish to proceed?',
			response: async (r: boolean) => {
				if (r) {
					await store.delete(`${item.type}:${item.name}`);
					await store.save();
					emit('store-update');
				}
			},
			buttonTextCancel: 'Cancel',
			buttonTextConfirm: 'Confirm'
		};
		modalStore.trigger(confirm);
	}
</script>

<script lang="ts">
	export let item: Item;
</script>

<button on:click={() => deleteResource(item)}>
	<i class="mdi mdi-delete text-red-300" />
</button>

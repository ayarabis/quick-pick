<script lang="ts" context="module">
	import { settingStore, store, type Item } from '$lib/app';
	import { modalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { emit } from '@tauri-apps/api/event';

	async function confirmDeeleteResource(item: Item) {
		const confirmDelete = (await settingStore.get('confirmDelete')) ?? false;
		console.log(confirmDelete);

		if (confirmDelete) {
			const confirm: ModalSettings = {
				type: 'confirm',
				title: 'Please Confirm',
				body: 'Are you sure you wish to proceed?',
				response: async (r: boolean) => {
					if (r) {
						deleteResource(item);
					}
				},
				buttonTextCancel: 'Cancel',
				buttonTextConfirm: 'Confirm'
			};
			modalStore.trigger(confirm);
		} else {
			deleteResource(item);
		}
	}

	async function deleteResource(item: Item) {
		await store.delete(`${item.type}:${item.name}`);
		await store.save();
		emit('store-update');
	}
</script>

<script lang="ts">
	export let item: Item;
</script>

<button on:click={() => confirmDeeleteResource(item)}>
	<i class="mdi mdi-delete text-red-300" />
</button>

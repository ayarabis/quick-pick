<script lang="ts" context="module">
	import { openResource, type Item } from '$lib/app';
	import { clipboard } from '@tauri-apps/api';
	async function copyContent(item: Item) {
		await clipboard.writeText(item.path);
	}

	async function executeAction(actionIndex: number, item: Item) {
		console.log(actionIndex);

		if (actionIndex) {
			copyContent(item);
		} else {
			openResource(item);
		}
	}
</script>

<script lang="ts">
	export let item: Item;
	export let disableHover = false;
	export let active = false;
	export let actionIndex = 0;
</script>

<button
	on:click={() => executeAction(actionIndex, item)}
	class="flex items-center justify-between w-full px-2 rounded-md cursor-pointer {disableHover
		? ''
		: 'hover:bg-surface-500'}  {active ? 'bg-primary-800' : ''}">
	<div class="whitespace-nowrap mr-2 flex items-center">
		<i class="mdi mdi-web mr-2" />
		{item.name}
	</div>
	<div>
		<button
			on:click|preventDefault|stopPropagation={() => openResource(item)}
			class="action ml-2 hover:text-secondary-500 {active && !actionIndex
				? 'text-primary-500 active'
				: ''}">
			<i class="mdi mdi-open-in-new" />
		</button>
		<button
			on:click|preventDefault|stopPropagation={() => copyContent(item)}
			class="action ml-2 hover:text-secondary-500 {active && !!actionIndex
				? 'text-primary-500 active'
				: ''}">
			<i class="mdi mdi-content-copy" />
		</button>
	</div>
</button>

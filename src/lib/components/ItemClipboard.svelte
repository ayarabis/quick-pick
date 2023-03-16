<script lang="ts" context="module">
	import type { Item } from '$lib/app';
	import { clipboard } from '@tauri-apps/api';
	async function executeAction(item: Item) {
		await clipboard.writeText(item.value);
	}
</script>

<script lang="ts">
	export let item: Item;
	export let disableHover = false;
	export let active = false;
</script>

<button
	on:click={() => executeAction(item)}
	class="flex items-center justify-between w-full px-2 rounded-md cursor-pointer {disableHover
		? ''
		: 'hover:bg-surface-500'}  {active ? 'bg-primary-800' : ''}">
	<div class="whitespace-nowrap mr-2 flex items-center">
		<i class="mdi mdi-clipboard mr-2" />
		<span class="line-clamp-1 whitespace-pre-wrap text-left">
			{item.name}
		</span>
	</div>
	<div>
		<button
			on:click|preventDefault|stopPropagation={() => executeAction(item)}
			class="action ml-2 hover:text-secondary-500 {active ? 'text-primary-500 active' : ''}">
			<i class="mdi mdi-content-copy" />
		</button>
	</div>
</button>

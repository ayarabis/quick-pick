<script lang="ts" context="module">
	import { openResource, type Item } from '$lib/app';
	import { clipboard } from '@tauri-apps/api';
	import { createEventDispatcher } from 'svelte';
	import DeleteBtn from './DeleteBtn.svelte';

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
	export let manage = false;
	export let actionIndex = 0;

	const dispatchEvent = createEventDispatcher();
</script>

{#if manage}
	<div class="card w-full mb-1 overflow-x-hidden h-7">
		<div class="text-left flex justify-between items-center pl-2 h-full">
			<div class="flex items-center">
				<i class="mdi mdi-web mr-2" />
				<span class="ml-1 whitespace-nowrap mr-2">{item.name}</span>
				<div class="text-sm line-clamp-1 whitespace-pre-wrap text-surface-400 mr-5">
					{item.path}
				</div>
			</div>
			<div class="flex gap-3 items-center sticky right-0 card px-2 h-full">
				<button on:click={() => dispatchEvent('edit')}>
					<i class="mdi mdi-pencil text-primary-400" />
				</button>
				<DeleteBtn {item} />
			</div>
		</div>
	</div>
{:else}
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
{/if}

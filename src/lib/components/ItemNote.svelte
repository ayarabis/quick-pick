<script lang="ts" context="module">
	import type { Item } from '$lib/app';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { createEventDispatcher } from 'svelte';
	import DeleteBtn from './DeleteBtn.svelte';

	async function executeAction(item: Item) {
		const label = `note-${item.id}`;
		const win = WebviewWindow.getByLabel(label);
		if (win) {
			win.setFocus();
			return;
		}
		new WebviewWindow(label, {
			width: 500,
			height: 490,
			decorations: false,
			url: `/note?name=${item.name}`,
			visible: true,
			alwaysOnTop: true
		});
	}
</script>

<script lang="ts">
	export let item: Item;
	export let disableHover = false;
	export let active = false;
	export let manage = false;

	const dispatchEvent = createEventDispatcher();
</script>

{#if manage}
	<div class="card w-full mb-1 overflow-x-hidden h-7">
		<div class="text-left flex justify-between items-center pl-2 h-full">
			<div class="flex items-center">
				<i class="mdi mdi-note-edit" />
				<span class="ml-1 whitespace-nowrap mr-2">{item.name}</span>
			</div>
			<div class="flex gap-3 items-center sticky right-0 card px-2">
				<button on:click={() => dispatchEvent('edit')}>
					<i class="mdi mdi-pencil text-primary-400" />
				</button>
				<DeleteBtn {item} />
			</div>
		</div>
	</div>
{:else}
	<button
		on:click={() => executeAction(item)}
		class="flex items-center justify-between w-full px-2 rounded-md cursor-pointer {disableHover
			? ''
			: 'hover:bg-surface-500'}  {active ? 'bg-primary-800' : ''}">
		<div class="whitespace-nowrap mr-2 flex items-center">
			<i class="mdi mdi-note-edit mr-2" />
			{item.name}
		</div>
		<div>
			<button
				on:click|preventDefault|stopPropagation={() => executeAction(item)}
				class="action ml-2 hover:text-secondary-500 {active
					? 'text-primary-500 active'
					: ''}">
				<i class="mdi mdi-pencil" />
			</button>
		</div>
	</button>
{/if}

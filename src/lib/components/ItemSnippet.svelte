<script lang="ts" context="module">
	import { showNotification, type Item } from '$lib/app';
	import { clipboard } from '@tauri-apps/api';
	import { createEventDispatcher } from 'svelte';
	import { HighlightAuto } from 'svelte-highlight';
	import DeleteBtn from './DeleteBtn.svelte';

	async function executeAction(item: Item) {
		await clipboard.writeText(item.content);
		showNotification('Snippet Copied!', item.name);
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
	<div class="card w-full mb-1 rounded-2xl">
		<div class="text-left flex justify-between items-center px-2">
			<div class="whitespace-nowrap">
				<i class="mdi mdi-code-braces mr-2" />
				{item.name}
			</div>
			<div class="flex gap-3 pt-1 items-center">
				<button class="copy" data-clipboard-target="#snippet_{item.id}">
					<i class="mdi mdi-content-copy" />
				</button>
				<button on:click={() => dispatchEvent('edit')}>
					<i class="mdi mdi-pencil text-primary-400" />
				</button>
				<DeleteBtn {item} />
			</div>
		</div>
		<div class="p-1">
			<HighlightAuto code={item.content} id="snippet_{item.id}" />
		</div>
	</div>
{:else}
	<button
		on:click={() => executeAction(item)}
		class=" w-full rounded-md cursor-pointer {disableHover
			? ''
			: 'hover:bg-surface-500'}  {active ? 'bg-primary-800' : ''}">
		<div class="flex items-center justify-between px-2">
			<div class="whitespace-nowrap mr-2 flex items-center">
				<i class="mdi mdi-code-braces mr-2" />
				{item.name}
			</div>
			<div>
				<button
					on:click|preventDefault|stopPropagation={() => executeAction(item)}
					class="action ml-2 hover:text-secondary-500 {active
						? 'text-primary-500 active'
						: ''}">
					<i class="mdi mdi-content-copy text-xl" />
				</button>
			</div>
		</div>
		<div class="p-1 text-left">
			<HighlightAuto code={item.content} id="snippet_{item.id}" />
		</div>
	</button>
{/if}

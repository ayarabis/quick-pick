<script lang="ts" context="module">
	async function executeAction(item: Item) {
		await clipboard.writeText(item.content);
		showNotification('Snippet Copied!', item.name);
	}
</script>

<script lang="ts">
	import { showNotification, type Item } from '$lib/app';
	import { clipboard } from '@tauri-apps/api';
	import { HighlightAuto } from 'svelte-highlight';

	export let item: Item;
	export let disableHover = false;
	export let active = false;
</script>

<button
	on:click={() => executeAction(item)}
	class=" w-full rounded-md cursor-pointer {disableHover ? '' : 'hover:bg-surface-500'}  {active
		? 'bg-primary-800'
		: ''}">
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

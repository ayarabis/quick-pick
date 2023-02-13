<script lang="ts" context="module">
	import type { Item } from '$lib/app';
	import { WebviewWindow } from '@tauri-apps/api/window';

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
</script>

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
			class="action ml-2 hover:text-secondary-500 {active ? 'text-primary-500 active' : ''}">
			<i class="mdi mdi-pencil" />
		</button>
	</div>
</button>

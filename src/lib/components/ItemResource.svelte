<script lang="ts" context="module">
	import { execCommand, extensionMap, openResource, type Item } from '$lib/app';
	import { Command } from '@tauri-apps/api/shell';
	const actions: { [key: string]: any } = {
		Folder: [
			{
				name: 'open folder',
				icon: 'mdi-folder-open',
				callback: (item: Item) => {
					openResource(item);
				}
			},
			{
				name: 'new terminal from folder',
				icon: 'mdi-console',
				callback: (item: Item) => {
					const cmd = new Command('open', ['-a', 'Terminal', '.'], {
						cwd: item.path
					});
					execCommand(cmd);
				}
			},
			{
				name: 'open to vscode',
				icon: 'mdi-microsoft-visual-studio-code',
				callback: (item: Item) => {
					const cmd = new Command('sh', ['/usr/local/bin/code', item.path]);
					execCommand(cmd);
				}
			}
		],
		File: [
			{
				name: 'open file',
				icon: 'mdi-open-in-new',
				callback: (item: Item) => {
					openResource(item);
				}
			}
		]
	};
</script>

<script lang="ts">
	export let item: Item;
	export let disableHover = false;
	export let itemHeight = 30;
	export let active = false;
	export let actionIndex = 0;

	async function executeAction() {
		actions[item.type][0].callback(item);
	}
</script>

<button
	on:click={executeAction}
	style="min-height: {itemHeight}px;"
	class="flex items-center justify-between w-full px-2 rounded-md cursor-pointer {disableHover
		? ''
		: 'hover:bg-surface-500'}  {active ? 'bg-primary-800' : ''}">
	<div class="whitespace-nowrap mr-2 flex items-center">
		{#if item.type == 'Folder'}
			<i class="mdi mdi-folder mr-2" />
		{:else}
			<i class="mdi {extensionMap[item.ext]} mr-2" />
		{/if}
		{item.name}
	</div>
	<div>
		{#if actions[item.type].length}
			{#each actions[item.type] as action, ai}
				<button
					on:click|preventDefault|stopPropagation={() => action.callback(item)}
					class="action ml-2 hover:text-secondary-500 {active && actionIndex == ai
						? 'text-primary-500'
						: ''}">
					<i class="mdi {action.icon}" />
				</button>
			{/each}
		{/if}
	</div>
</button>

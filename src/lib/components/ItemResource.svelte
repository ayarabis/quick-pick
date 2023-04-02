<script lang="ts" context="module">
	import { execCommand, extensionMap, openResource, type Item } from '$lib/app';
	import { Command } from '@tauri-apps/api/shell';
	import { createEventDispatcher } from 'svelte';
	import DeleteBtn from './DeleteBtn.svelte';

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
	export let active = false;
	export let manage = false;
	export let actionIndex = 0;

	const dispatchEvent = createEventDispatcher();

	async function executeAction() {
		actions[item.type][0].callback(item);
	}
</script>

{#if manage}
	<div class="card w-full mb-1 overflow-x-hidden h-7">
		<div class="text-left flex justify-between items-center pl-2 relative h-full">
			<div class="flex items-center">
				{#if item.type == 'Folder'}
					<i class="mdi mdi-folder mr-2" />
				{:else}
					<i class="mdi mr-2 {extensionMap[item.ext || ''] || 'mdi-file-question'}" />
				{/if}
				<div class="whitespace-nowrap mr-2">{item.name}</div>
				<span class="text-sm line-clamp-1 whitespace-pre-wrap text-surface-400 mr-5">
					{item.path}
				</span>
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
		on:click={executeAction}
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
{/if}

<script lang="ts">
	import '$lib/app';
	import { journalStore } from '$lib/app';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { AppShell, Toast } from '@skeletonlabs/skeleton';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	import groupBy from 'lodash.groupby';
	import moment from 'moment';
	import { onMount } from 'svelte';
	import {  storeLightSwitch } from '@skeletonlabs/skeleton';

	let pinWindow = false;

	const eventListeners: UnlistenFn[] = [];

	let items: { [key: string]: any[] } = {};

	onMount(async () => {
		await loadItems();

		eventListeners.push(await listen('update-journal', loadItems));
	});

	async function loadItems() {
		const activities = (await journalStore.values()).sort((a: any, b: any) => {
			if (a.timestamp > b.timestamp) return 1;
			if (a.timestamp < b.timestamp) return -1;
			return 0;
		});
		items = groupBy(activities, (e: any) => moment(e.timestamp).format('MMM DD'));
	}

	function togglePin() {
		pinWindow = !pinWindow;
		appWindow.setAlwaysOnTop(pinWindow);
	}
</script>

<Toast padding="p-2" buttonDismiss="btn-sm btn-round-full bg-white shadow-md p-1" />
<AppShell>
	<svelte:fragment slot="header">
		<TitleBar title="Quick Pick - Journal">
			<svelte:fragment slot="lead-ations">
				<button class="titlebar-button" on:click={togglePin}>
					<i class="mdi {pinWindow ? 'mdi-pin-off text-primary-600' : 'mdi-pin'}" />
				</button>
			</svelte:fragment>
		</TitleBar>
	</svelte:fragment>
	{#if Object.keys(items).length}
		<div class="p-2 h-full dark">
			<span class="divider-vertical h-full absolute left-[13px]" />
			{#each Object.keys(items) as day}
				{@const activities = items[day]}
				<div class="flex items-center gap-5 mb-1">
					<div class="h-3 w-3 rounded-full bg-primary-500 z-10" />
					<span class="chip bg-primary-500">{day}</span>
				</div>
				{#each activities as item}
					<div class="card rounded-md px-2 pb-2 mb-1 ml-3">
						<span class="text-sm text-surface-300">{item.timestamp}</span>
						<div>{item.activity}</div>
					</div>
				{/each}
				<div class="mb-2" />
			{/each}
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center h-full text-surface-300 text-2xl">
			<i class="mdi mdi-note-edit" />
			Start adding your activity
		</div>
	{/if}
</AppShell>

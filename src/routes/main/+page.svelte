<script lang="ts">
	import { appStore, canHide, journalStore, showNotification, store, type Item } from '$lib/app';
	import ItemNote from '$lib/components/ItemNote.svelte';
	import ItemResource from '$lib/components/ItemResource.svelte';
	import ItemSnippet from '$lib/components/ItemSnippet.svelte';
	import ItemUrl from '$lib/components/ItemUrl.svelte';
	import QuickApps from '$lib/components/QuickApps.svelte';
	import { AppBar, Modal } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { basename } from '@tauri-apps/api/path';
	import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
	import debounce from 'lodash.debounce';
	import moment from 'moment';
	import { onDestroy, onMount } from 'svelte';
	import { v4 as uuidv4 } from 'uuid';

	let items: Item[] = [];
	let itemHeight = 30;
	let maxItemDislay = 13;
	let visibleFirst = 0;
	let panelHeight = 0;
	let disableHover = true;
	let itemIndex = 0;
	let actionIndex = 0;
	let showApps = true;

	let search: string = '';
	let searchItems: Item[] = [];

	let fileHover = false;

	const eventListeners: UnlistenFn[] = [];

	onMount(async () => {
		eventListeners.push(await listen('store-update', reloadData));

		eventListeners.push(
			await store.onChange(async (_) => {
				reloadData();
			})
		);

		await reloadData();

		const dropzone = WebviewWindow.getByLabel('dropzone');
		if (dropzone) {
			eventListeners.push(
				await dropzone?.onFileDropEvent((e) => {
					if (['cancel', 'drop'].includes(e.payload.type)) {
						dropzone.hide();
						appWindow.setFocus();
						appWindow.setAlwaysOnTop(true);
						fileHover = false;
						$canHide = true;
					}
				})
			);
		}

		eventListeners.push(
			await appWindow.onFocusChanged((f) => {
				if (f.payload) {
					document.getElementById('searchbox')?.focus();
				} else {
					if ($canHide) {
						invoke('hide_window');
					}
					clearItems();
				}
			})
		);
	});

	onDestroy(async () => {
		eventListeners.forEach(async (e) => e());
	});

	async function reloadData() {
		items = await store.values();
		filterItems();
	}

	function checkVisible(el: HTMLElement) {
		var rect = el.getBoundingClientRect();
		var viewHeight = Math.max(document.documentElement.clientHeight, window.innerHeight);
		return !(rect.bottom < 0 || rect.top - viewHeight >= 0);
	}

	function onKeyUp(event: KeyboardEvent) {
		if (event.key == 'Enter') {
			const el: HTMLButtonElement | null = document.querySelector(
				`.item.active .action:nth-child(${actionIndex + 1})`
			);
			el?.click();
			appWindow.hide();
		}
	}

	async function onKeyDown(event: KeyboardEvent) {
		const code = event.key;

		if (code == 'Escape') {
			if (search) {
				search = '';
				filterItems();
			} else {
				await appWindow.hide();
			}
		}
		switch (code) {
			case 'ArrowUp':
				if (itemIndex != 0) {
					--itemIndex;
					if (itemIndex <= visibleFirst) {
						const panel = document.getElementById('panel');
						const scroll = panel!.scrollTop - itemHeight;

						document.getElementById('panel')?.scrollTo({ top: scroll });
					}
					const el = document.getElementById(`item_${itemIndex}`);
					if (!checkVisible(el!)) {
						el?.scrollIntoView();
						visibleFirst = itemIndex;
					}
				}
				actionIndex = 0;
				disableHover = true;
				event.preventDefault();
				event.stopImmediatePropagation();
				break;
			case 'ArrowDown':
				if (itemIndex < searchItems.length - 1) {
					++itemIndex;
					if (itemIndex >= maxItemDislay) {
						let offset = (itemIndex + 1 - maxItemDislay) * itemHeight;
						document.getElementById('panel')?.scrollTo({ top: offset });

						visibleFirst = itemIndex - maxItemDislay;
						const el = document.getElementById(`item_${itemIndex}`);
						if (!checkVisible(el!)) {
							el?.scrollIntoView();
						}
					}
				}
				actionIndex = 0;
				disableHover = true;
				event.preventDefault();
				event.stopImmediatePropagation();
				break;
			case 'Tab':
				if (event.shiftKey) {
					if (actionIndex != 0) actionIndex--;
				} else {
					const actions = document.querySelectorAll('#panel .item.active .action');

					if (actionIndex < actions.length - 1) actionIndex++;
				}
				event.preventDefault();
				event.stopImmediatePropagation();
		}
	}

	const filterItems = debounce(async () => {
		itemIndex = 0;
		actionIndex = 0;

		const main = WebviewWindow.getByLabel('main');
		const size = (await main!.outerSize()).toLogical(await main!.scaleFactor());
		if (search) {
			let _search = search.replace(/f:|s:|w:|n:/gi, '');

			searchItems = items.filter((e) => {
				const re = new RegExp(_search, 'gi');
				let result = re.test(e.name);
				if (search.startsWith('f:')) {
					result = result && ['File', 'Folder'].includes(e.type);
				} else if (search.startsWith('w:')) {
					result = result && e.type == 'WebURL';
				} else if (search.startsWith('s:')) {
					result = result && e.type == 'Snippet';
				} else if (search.startsWith('n:')) {
					result = result && e.type == 'Note';
				}

				return result;
			});
			if (!searchItems.length) {
				if (search.startsWith('http')) {
					try {
						new URL(search);
						searchItems.push({
							name: 'Save as Web URL',
							type: 'Action',
							callback: saveUrl
						});
					} catch (_) {}
				}
				searchItems.push(
					{
						name: 'Create a note',
						type: 'Action',
						callback: createNote
					},
					{
						name: 'Record activity',
						type: 'Action',
						callback: recordActivity
					}
				);
				searchItems = searchItems;
			}
		} else {
			searchItems = [];
		}

		let maxHeight = maxItemDislay * itemHeight;
		panelHeight = searchItems.length * itemHeight;
		if (searchItems.length > maxItemDislay) panelHeight = maxHeight;
		if (panelHeight) {
			panelHeight += 10;
			showApps = false;
		} else {
			panelHeight = 65;
			showApps = true;
		}
		setTimeout(async () => {
			const docHeight = document.getElementById('item-list-container')?.clientHeight ?? 0;
			if (docHeight > maxHeight && searchItems.length) {
				panelHeight = maxHeight;
			} else if (searchItems.length) {
				panelHeight = docHeight + 9;
			}

			size.height = 92 + panelHeight;
			await main?.setSize(size);
		}, 10);
	}, 100);

	async function createNote() {
		const win = new WebviewWindow('create-note', {
			width: 500,
			height: 490,
			decorations: false,
			url: `/note?name=${search}`
		});

		await win.show();
		await appWindow.hide();
	}

	async function recordActivity() {
		await journalStore.set(uuidv4(), {
			timestamp: moment(),
			activity: search
		});
		await journalStore.save();
		emit('update-journal');
	}

	async function saveUrl() {
		const path = search;
		const name = await basename(path);
		const item = {
			id: uuidv4(),
			type: 'WebURL',
			name,
			path
		};
		await store.set(`WebURL:${name}`, item);
		await store.save();
		await emit('store-update');
		showNotification('WebURL Saved', path);
		clearItems();
	}

	async function openManager() {
		const manageWindow = WebviewWindow.getByLabel('manage');
		manageWindow?.show();
	}

	async function clearItems() {
		search = '';
		searchItems = [];
		itemIndex = 0;
		filterItems();
	}

	let dragIndex: number = -1;
	let onDropZone = false;
	function onAppDrag(index: number) {
		dragIndex = index;
	}

	async function openJournal() {
		const win =
			WebviewWindow.getByLabel('journal') ??
			new WebviewWindow('journal', {
				width: 500,
				height: 490,
				decorations: false,
				url: `/journal`
			});

		await win.show();
		await appWindow.hide();
	}

	async function onAppDrop(e: DragEvent) {
		const data = JSON.parse(e.dataTransfer?.getData('text/plain') ?? '{}');

		await appStore.delete(`${data.index}`);
		await appStore.save();

		appWindow.emit('on-remove-app');

		onDropZone = false;
	}

	async function showDropZone() {
		if (fileHover || dragIndex != -1) return;

		$canHide = false;
		await appWindow.setAlwaysOnTop(false);
		const size = await appWindow.outerSize();
		const pos = await appWindow.outerPosition();

		const win = WebviewWindow.getByLabel('dropzone');

		await win?.setSize(size);
		await win?.setPosition(pos);
		await win?.setFocus();
		fileHover = true;
	}
</script>

<svelte:head>
	<style>
		#page {
			overflow-x: hidden;
		}
	</style>
</svelte:head>

<svelte:window
	on:dragenter={showDropZone}
	on:keydown={onKeyDown}
	on:keyup={onKeyUp}
	on:mousemove={() => (disableHover = false)} />

<Modal />
<AppBar padding="p-2">
	<svelte:fragment slot="lead">
		<strong class="text-xl uppercase"> Quick Pick</strong>
		<span class="divider-vertical h-4 ml-4" />
	</svelte:fragment>
	<svelte:fragment slot="trail">
		<button on:click={openJournal}>
			<i class="mdi mdi-notebook" />
		</button>
		<button on:click={openManager}>
			<i class="mdi mdi-menu" />
		</button>
	</svelte:fragment>
</AppBar>
<div class="p-1">
	{#if dragIndex != -1}
		<div
			on:drop={onAppDrop}
			on:dragover|preventDefault={(e) => false}
			on:dragenter={(e) => (onDropZone = true)}
			on:dragleave={(e) => (onDropZone = false)}
			class="w-full relative flex items-center pr-2 border-2  border-dashed {onDropZone
				? 'border-red-300'
				: 'border-primary-500'}">
			<i
				class="mdi mdi-delete {onDropZone
					? 'text-red-300'
					: 'text-primary-500'} absolute left-3 top-[50%] translate-y-[-50%]" />
			<input
				disabled
				type="text"
				class="!bg-transparent ml-7 text-center pointer-events-none transition-none !text-opacity-50"
				value="drop here to remove shortcut" />
		</div>
	{:else}
		<div class="w-full card relative flex items-center pr-2 border-2 border-transparent">
			<i class="mdi mdi-magnify absolute left-3 top-[50%] translate-y-[-50%]" />
			<input
				bind:value={search}
				on:input={filterItems}
				id="searchbox"
				type="text"
				class="!bg-transparent ml-7 "
				autocomplete="off"
				autocapitalize="off"
				autocorrect="off" />
			{#if search}
				<button on:click={clearItems}>
					<i class="mdi mdi-backspace" />
				</button>
			{/if}
		</div>
	{/if}
</div>

<div id="panel" class="!overflow-x-auto pb-[5px]" style="max-height: {panelHeight}px;">
	{#if showApps}
		<QuickApps {onAppDrag} />
	{/if}
	<div class="px-1" id="item-list-container">
		{#each searchItems as item, index}
			{@const active = itemIndex == index}
			<!-- svelte-ignore a11y-mouse-events-have-key-events -->
			<div
				id="item_{index}"
				on:mouseover={() => (itemIndex = index)}
				class="item"
				class:active>
				{#if item.type == 'Snippet'}
					<ItemSnippet {item} {itemHeight} {active} {disableHover} />
				{:else if ['File', 'Folder'].includes(item.type)}
					<ItemResource {item} {itemHeight} {active} {disableHover} {actionIndex} />
				{:else if item.type == 'WebURL'}
					<ItemUrl {item} {itemHeight} {active} {disableHover} />
				{:else if item.type == 'Note'}
					<ItemNote {item} {itemHeight} {active} {disableHover} />
				{:else if (item.type = 'Action')}
					<button
						on:click|preventDefault|stopPropagation={item.callback}
						style="min-height: {itemHeight}px;"
						class="action flex items-center w-full px-2 rounded-md cursor-pointer {disableHover
							? ''
							: 'hover:bg-surface-500'}  {active ? 'bg-primary-800' : ''}">
						<i class="mdi {item.icon}" />
						{item.name}
					</button>
				{/if}
			</div>
		{/each}
	</div>
</div>

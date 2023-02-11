<script lang="ts">
	import { appStore, canHide, execCommand, type QuickApp } from '$lib/app';
	import { parse } from '@plist/plist';
	import { open } from '@tauri-apps/api/dialog';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import { appDataDir, basename } from '@tauri-apps/api/path';
	import { Command } from '@tauri-apps/api/shell';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';
	import { v4 as uuidv4 } from 'uuid';

	export let onAppDrag: (index: number) => void;

	let items: { app: QuickApp | null; id: string }[] = [];

	const eventListeners: UnlistenFn[] = [];

	onMount(async () => {
		await loadApps();
		eventListeners.push(await appWindow.listen('on-remove-app', async () => await loadApps()));
	});

	onDestroy(() => {
		eventListeners.forEach((e) => e());
	});

	async function loadApps() {
		items = [];
		for (let i = 0; i < 10; i++) {
			const app: QuickApp | null = await appStore.get(`${i}`);
			items.push({
				app,
				id: uuidv4()
			});
		}
		items = items;
	}

	let dropTarget: number | null;
	function onSlotDrag(e: DragEvent, index: number) {
		e.dataTransfer?.setData(
			'text/plain',
			JSON.stringify({
				index
			})
		);
		onAppDrag(index);
	}

	function showTarget(e: DragEvent, index: number) {
		dropTarget = index;
		(e.target as HTMLDivElement).classList.add('drop-target');
	}
	function removeTarget(e: DragEvent) {
		dropTarget = null;
		(e.target as HTMLDivElement).classList.remove('drop-target');
	}

	function onDragEnd(e: DragEvent) {
		dropTarget = null;
		onAppDrag(-1);
	}

	async function onSlotDrop(e: DragEvent, slotIndex: number) {
		e.preventDefault();
		const data = JSON.parse(e.dataTransfer?.getData('text/plain') ?? '{}');
		const itemIndex = data.index;

		const [item] = items.splice(itemIndex, 1, { app: null, id: uuidv4() });
		const slotApp = items[slotIndex];
		items[slotIndex] = item;
		if (slotApp && slotIndex != itemIndex) {
			items[itemIndex] = slotApp;
		}
		items = items;

		for (let i = 0; i < items.length; i++) {
			const e = items[i];
			await appStore.set(`${i}`, e.app);
		}
		await appStore.save();
	}

	async function selectApp(index: number) {
		let app = items[index].app;
		if (app) {
			new Command('open', app.app).execute();
			return;
		}
		$canHide = false;
		const selected = (await open({
			multiple: false,
			defaultPath: '/Applications'
		})) as string | undefined;

		if (selected) {
			let path = `${selected}/Contents`;
			let appName = await basename(selected);
			let outIconName = appName.replace(' ', '').toLowerCase().split('.')[0];

			let content = '';
			var cmdIcn = new Command('cat', `Info.plist`, { cwd: path });
			cmdIcn.stdout.on('data', (data) => {
				content += data;
			});
			cmdIcn.on('close', async () => {
				let val: { [key: string]: any } = parse(content)?.valueOf() as object;
				let icon = val.CFBundleIconFile as string;
				if (!icon.endsWith('.icns')) icon += '.icns';
				const iconPath = `${path}/Resources/${icon}`;
				const appDataPath = await appDataDir();

				const cmdPng = new Command(
					'sips',
					['-s', 'format', 'png', iconPath, '--out', `${appDataPath + outIconName}.png`],
					{ cwd: `${path}/Resources` }
				);
				await execCommand(cmdPng);

				const src = convertFileSrc(`${appDataPath + outIconName}.png`);

				app = {
					app: selected,
					iconName: outIconName,
					iconPath: src
				};

				await appStore.set(index.toString(), app);
				await appStore.save();
				loadApps();
			});
			await execCommand(cmdIcn);
		}

		$canHide = true;
	}
</script>

<svelte:window on:dragend={onDragEnd} />
<div class="flex items-center justify-evenly pt-1" on:dragenter={removeTarget}>
	{#each items as item, index (item.id)}
		{@const app = item.app}
		<div
			class="slot relative h-12 w-12 bg-primary-500 cursor-pointer border-2 border-transparent flex items-center justify-center card rounded-md
            {dropTarget == index ? 'border-primary-800' : ''}">
			<button
				class="w-full h-full p-1"
				draggable={app != null}
				on:drop={(e) => onSlotDrop(e, index)}
				on:dragenter|stopPropagation={(e) => showTarget(e, index)}
				on:dragover|preventDefault={(e) => false}
				on:dragstart={(e) => onSlotDrag(e, index)}
				on:dragend={() => onDragEnd}
				on:click={() => selectApp(index)}>
				{#if app}
					<img class="pointer-events-none" src={app.iconPath} alt={app.iconName} />
				{:else}
					<i class="mdi mdi-plus text-xl pointer-events-none" />
				{/if}
			</button>
		</div>
	{/each}
</div>

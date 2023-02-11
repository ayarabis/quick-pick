<script lang="ts">
	import { extensionMap, store, type Item } from '$lib/app';
	import '$lib/app.postcss';
	import { emit, TauriEvent, type UnlistenFn } from '@tauri-apps/api/event';
	import {
		isPermissionGranted,
		requestPermission,
		sendNotification
	} from '@tauri-apps/api/notification';
	import { basename } from '@tauri-apps/api/path';
	import { appWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';
	import { HtmlTag } from 'svelte/internal';
	import { v4 as uuidv4 } from 'uuid';

	let unilisten: UnlistenFn | null;

	onMount(async () => {
		unilisten = await appWindow.listen(TauriEvent.WINDOW_FILE_DROP, async (event) => {
			const extensions = Object.keys(extensionMap);
			const payload = event.payload as string[];
			const added: any[] = [];
			for (let i = 0; i < payload.length; i++) {
				const path = payload[i];
				const ext = path.split('.').reverse()[0];
				const name = await basename(path);
				const type = extensions.includes(ext) ? 'File' : 'Folder';
				const item: Item = {
					id: uuidv4(),
					name,
					path,
					ext,
					type
				};
				await store.set(`${type}:${name}`, item);
				added.push(item);
			}

			await store.save();
			if (added.length) {
				if (await isPermissionGranted()) {
					sendNotification({
						icon: 'icon.png',
						title: 'New Resource Added',
						body: added.reduce((a, b) => {
							a += (b.type == 'File' ? '📄' : '📁') + `${b.name} \n`;
							return a;
						}, '\n')
					});
				} else {
					await requestPermission();
				}
				emit('store-update');
			}
		});
	});

	onDestroy(() => unilisten?.());
</script>

<svelte:head>
	<style>
		html,
		body {
			background: transparent !important;
		}
	</style>
</svelte:head>

<div class="dropzone">
	<span class="font-bold">Drop file/folder to add as resource</span>
</div>

<style>
	.dropzone {
		background-color: rgba(0, 0, 0, 0.3);
		display: flex;
		justify-content: center;
		align-items: center;
		color: aliceblue;
		font-size: 20pt;
		height: 100%;
		width: 100%;
	}
</style>

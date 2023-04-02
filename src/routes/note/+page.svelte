<script lang="ts">
	import { page } from '$app/stores';
	import '$lib/app';
	import { store, type Item } from '$lib/app';
	import Header from '$lib/components/Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { AppShell, Toast, toastStore, type ToastSettings } from '@skeletonlabs/skeleton';
	import { emit } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	import debounce from 'lodash.debounce';
	import { onMount } from 'svelte';
	import { v4 as uuidv4 } from 'uuid';

	let item: Item | null;
	let name = $page.url.searchParams.get('name') ?? '';

	let editor: HTMLElement;
	let quill: any;
	let saved = false;

	onMount(async () => {
		const { default: Quill } = await import('quill');

		quill = new Quill(editor, {
			modules: {
				toolbar: '#toolbar-container'
			},
			theme: 'snow'
		});

		const debounceSave = debounce(() => {
			save();
			saved = true;
		}, 200);

		quill.on('text-change', () => {
			saved = false;
			debounceSave();
		});

		item = await store.get(`Note:${name}`);

		if (item) {
			quill.setContents(item?.content);
		}
	});

	async function save() {
		const data = {
			id: item?.id ?? uuidv4(),
			name,
			content: quill.getContents(),
			type: 'Note'
		};

		if (item) {
			await store.delete(`Note:${item.name}`);
		}
		await store.set(`Note:${name}`, data);
		await store.save();
		await emit('store-update');

		// const toast: ToastSettings = {
		// 	message: 'Saved!',
		// 	preset: 'success',
		// 	autohide: true,
		// 	timeout: 1000
		// };
		// toastStore.trigger(toast);
	}

	function close() {
		appWindow.close();
	}
</script>

<svelte:head>
	<style>
		.dark .ql-toolbar .ql-stroke {
			fill: none;
			stroke: #fff;
		}

		.dark .ql-toolbar .ql-fill {
			fill: #fff;
			stroke: none;
		}

		.dark .ql-toolbar .ql-picker {
			color: #fff;
		}

		.ql-toolbar,
		.ql-container {
			border: none !important;
		}
		.app-bar-lead {
			width: 100%;
		}
	</style>
</svelte:head>

<Toast padding="p-2" buttonDismiss="btn-sm btn-round-full bg-white shadow-md p-1" />
<AppShell>
	<svelte:fragment slot="header">
		<TitleBar title="Quick Pick - Note" />
		<Header>
			<svelte:fragment slot="title">
				<input type="text" class="!bg-transparent" bind:value={name} />
			</svelte:fragment>
		</Header>
	</svelte:fragment>
	<div class="dark:bg-surface-700 h-[100%] overflow-hidden">
		<div id="toolbar-container" class="dark:!text-white">
			<i class="mdi mdi-content-save" />
			<span class="ql-formats">
				<select class="ql-font" />
				<select class="ql-size" />
			</span>
			<span class="ql-formats">
				<button class="ql-bold" />
				<button class="ql-italic" />
				<button class="ql-underline" />
				<button class="ql-strike" />
			</span>
			<span class="ql-formats">
				<select class="ql-color" />
				<select class="ql-background" />
			</span>
			<span class="ql-formats">
				<button class="ql-script" value="sub" />
				<button class="ql-script" value="super" />
			</span>
			<span class="ql-formats">
				<button class="ql-header" value="1" />
				<button class="ql-header" value="2" />
				<button class="ql-blockquote" />
				<button class="ql-code-block" />
			</span>
			<span class="ql-formats">
				<button class="ql-list" value="ordered" />
				<button class="ql-list" value="bullet" />
				<button class="ql-indent" value="-1" />
				<button class="ql-indent" value="+1" />
			</span>
			<span class="ql-formats">
				<button class="ql-direction" value="rtl" />
				<select class="ql-align" />
			</span>
			<span class="ql-formats">
				<button class="ql-link" />
				<button class="ql-image" />
				<button class="ql-video" />
				<button class="ql-formula" />
			</span>
			<span class="ql-formats">
				<button class="ql-clean" />
			</span>
		</div>
		<hr />
		<div bind:this={editor} class="h-full" />
	</div>
	<svelte:fragment slot="footer">
		<div class="flex justify-end h-5 px-2">
			{#if saved}
				<div class="text-secondary-500 flex">
					<i class="mdi mdi-content-save text-sm mr-2 " />
					<i class="mdi mdi-check text-sm mr-2 " />
					<span class="text-sm ">saved</span>
				</div>
			{/if}
		</div>
	</svelte:fragment>
</AppShell>

<style>
	@import 'https://cdn.quilljs.com/1.3.6/quill.snow.css';
</style>

<script lang="ts">
	import { settingStore } from '$lib/app';
	import '$lib/app.postcss';
	import '@mdi/font/css/materialdesignicons.min.css';
	import '@skeletonlabs/skeleton/styles/all.css';
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/themes/theme-modern.css';
	import '@skeletonlabs/skeleton/themes/theme-rocket.css';
	import '@skeletonlabs/skeleton/themes/theme-seafoam.css';
	import '@skeletonlabs/skeleton/themes/theme-vintage.css';
	import '@skeletonlabs/skeleton/themes/theme-sahara.css';
	import '@skeletonlabs/skeleton/themes/theme-hamlindigo.css';
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	import '@skeletonlabs/skeleton/themes/theme-crimson.css';
	import { onDestroy, onMount } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';

	let theme: string;
	const eventListeners: UnlistenFn[] = [];
	onMount(async () => {
		loadTheme();
		eventListeners.push(await listen('on-theme-change', loadTheme));
	});

	onDestroy(() => {
		eventListeners.forEach((e) => e());
	});

	async function loadTheme() {
		theme = (await settingStore.get('theme')) ?? 'crimson';
	}
</script>

<div data-theme={theme} class="h-full">
	<slot />
</div>

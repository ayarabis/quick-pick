<script lang="ts">
	import { settingStore } from '$lib/app';
	import { emit } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	export let settings: any = {};

	onMount(async () => {
		settings.theme = await settingStore.get('theme');
	});

	async function setTheme(event: Event & { currentTarget: EventTarget & HTMLSelectElement }) {
		const value = (event.target as HTMLSelectElement).value;
		await settingStore.set('theme', value);
		await settingStore.save();
		settings.theme = value;

		emit('on-theme-change');
	}
</script>

<section>
	<div class="flex justify-between items-center">
		<span class=" whitespace-nowrap">System Theme</span>
		<select id="theme-select" bind:value={settings.theme} on:change={setTheme}>
			<option value="crimson">Crimson</option>
			<option value="hamlindigo">Hamlindigo</option>
			<option value="modern">Modern</option>
			<option value="rocket">Rocket</option>
			<option value="sahara">Sahara</option>
			<option value="seafoam">Seafoam</option>
			<option value="seasonal">Seasonal</option>
			<option value="skeleton">Skeleton</option>
			<option value="vintage">Vintage</option>
		</select>
	</div>
</section>

<style>
	#theme-select {
		max-width: 300px;
	}
</style>

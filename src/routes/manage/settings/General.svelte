<script lang="ts">
	import { settingStore } from '$lib/app';
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	export let settings: any = {};

	onMount(async () => {
		const entries = await settingStore.entries();
		entries.forEach((e) => {
			settings[e[0]] = e[1];
		});
	});

	async function saveSettings() {
		for await (const k of Object.keys(settings)) {
			await settingStore.set(k, settings[k]);
		}
		await settingStore.save();
	}
</script>

<section>
	<div class="flex justify-between items-center">
		<span class=" whitespace-nowrap">Confirm Deletion</span>
		<SlideToggle name="slide" bind:checked={settings.confirmDelete} on:change={saveSettings} />
	</div>
</section>

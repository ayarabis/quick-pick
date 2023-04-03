<script lang="ts">
	import { appStore, journalStore, store } from '$lib/app';
	import { invoke } from '@tauri-apps/api';
	import { github } from 'svelte-highlight/styles';

	invoke('init_spotlight_window');

	store.save();
	appStore.save();
	journalStore.save();
</script>

<svelte:head>
	{@html github}
	<style>
		pre {
			padding: 0px !important;
			background: transparent !important;
		}
	</style>
</svelte:head>
<slot />

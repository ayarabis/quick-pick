<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	export let title: string = 'Quick Pick';

	export let hideActions: boolean = false;

	appWindow.listeners;

	async function windowAction(action: string) {
		switch (action) {
			case 'min':
				appWindow.minimize();
				break;
			case 'max':
				appWindow.toggleMaximize();
				break;
			case 'close':
				appWindow.hide();
				break;
		}
	}
</script>

<div
	data-tauri-drag-region
	class="flex items-center justify-between pl-2 bg-surface-800 cursor-move text-white">
	{title}
	<div>
		<slot name="lead-actions" />
		{#if !hideActions}
			<button class="titlebar-button" on:click={() => windowAction('min')}>
				<i class="mdi mdi-window-minimize" />
			</button>
			<button class="titlebar-button" on:click={() => windowAction('max')}>
				<i class="mdi mdi-window-maximize" />
			</button>
			<button class="titlebar-button" on:click={() => windowAction('close')}>
				<i class="mdi mdi-window-close" />
			</button>
		{/if}
		<slot name="trail-actions" />
	</div>
</div>

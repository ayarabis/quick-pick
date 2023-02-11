<script lang="ts">
	import { canHide } from '$lib/app';
	import { modalStore } from '@skeletonlabs/skeleton';
	import { open } from '@tauri-apps/api/dialog';
	import { basename } from '@tauri-apps/api/path';
	import { onMount } from 'svelte';

	export let parent: any;
	export let formData: any = {};
	export let fileFolder = false;

	onMount(() => {
		formData.type = formData.type || 'Folder';
	});

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		modalStore.close();
	}
	const cBase = 'space-y-4';
	const cForm = 'border border-surface-500 p-4 space-y-4 rounded-container-token';

	async function selectPath() {
		$canHide = false;
		const selected = await open({
			multiple: false,
			directory: formData.type == 'Folder',
			defaultPath: '~/'
		});
		if (selected) {
			const folder = await basename(selected as string);
			formData.path = selected;
			if (!formData.name) formData.name = folder;
		}
		$canHide = true;
	}

	async function checkUrl() {
		try {
			new URL(formData.path);
			const resource = await basename(formData.path);
			if (!formData.name) formData.name = resource;
		} catch (_) {}
	}

	function reset() {
		formData.name = '';
		formData.path = '';
	}
</script>

<div class="modal-example-form {cBase}">
	<form class="modal-form {cForm}">
		{#if fileFolder}
			<label>
				<span>Type</span>
				<select bind:value={formData.type} on:change={reset}>
					<option value="File">File</option>
					<option value="Folder">Folder</option>
				</select>
			</label>
		{/if}
		<label>
			<span>Name</span>
			<input type="text" bind:value={formData.name} placeholder="Resource name" />
		</label>
		{#if formData.type == 'Snippet'}
			<label>
				<span>Content</span>
				<textarea
					rows="5"
					id="code"
					bind:value={formData.content}
					placeholder="Hello World"
					autocomplete="off"
					autocapitalize="off"
					autocorrect="off" />
			</label>
		{:else if formData.type == 'WebURL'}
			<label>
				<span>URL</span>
				<input
					type="text"
					bind:value={formData.path}
					placeholder="https://"
					on:keyup={checkUrl}
					autocomplete="off"
					autocapitalize="off"
					autocorrect="off" />
			</label>
		{:else}
			<label>
				<span>Path</span>
				<div class="flex items-center bg-surface-700 pr-1">
					<input
						readonly
						type="text"
						bind:value={formData.path}
						placeholder="Click select"
						class="!bg-transparent"
						autocomplete="off"
						autocapitalize="off"
						autocorrect="off" />
					<button on:click={selectPath} class="btn btn-sm btn-ghost-primary"
						>Select</button>
				</div>
			</label>
		{/if}
	</form>
	<footer class="modal-footer {parent.regionFooter}">
		<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}
			>{parent.buttonTextCancel}</button>
		<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Submit</button>
	</footer>
</div>

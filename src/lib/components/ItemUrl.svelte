<script lang="ts">
	import { openResource, type Item } from '$lib/app';

	export let item: Item;
	export let disableHover = false;
	export let itemHeight = 30;
	export let active = false;
</script>

<button
	on:click={() => openResource(item)}
	style="min-height: {itemHeight}px;"
	class="flex items-center justify-between w-full px-2 rounded-md cursor-pointer {disableHover
		? ''
		: 'hover:bg-surface-500'}  {active ? 'bg-primary-800' : ''}">
	<div class="whitespace-nowrap mr-2 flex items-center">
		<i class="mdi mdi-web mr-2" />
		{item.name}
	</div>
	<div>
		<button
			on:click|preventDefault|stopPropagation={() => openResource(item)}
			class="action ml-2 hover:text-secondary-500 {active ? 'text-primary-500 active' : ''}">
			<i class="mdi mdi-open-in-new" />
		</button>
	</div>
</button>

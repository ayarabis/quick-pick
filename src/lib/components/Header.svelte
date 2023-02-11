<script lang="ts">
	import { AppBar, LightSwitch } from '@skeletonlabs/skeleton';

	export let title: string = '';
</script>

<AppBar>
	<svelte:fragment slot="lead">
		<slot name="action" />
		<slot name="title">
			<strong class="text-xl uppercase">{title}</strong>
		</slot>
		<!-- <span class="divider-vertical h-4 ml-4" /> -->
	</svelte:fragment>
	<!-- <svelte:fragment slot="trail">
		<LightSwitch />
	</svelte:fragment> -->
</AppBar>

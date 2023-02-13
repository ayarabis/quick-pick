<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';

	import { AppShell } from '@skeletonlabs/skeleton';
	import Theme from './Theme.svelte';

	const settings: any = {};

	const sections = [
		{
			title: 'Theme',
			component: Theme
		}
	];
</script>

<AppShell>
	<svelte:fragment slot="header">
		<TitleBar />
		<Header title="Quick Pick">
			<svelte:fragment slot="action">
				<a href="/manage" class="mr-2">
					<i class="mdi mdi-arrow-left text-2xl" />
				</a>
			</svelte:fragment>
		</Header>
	</svelte:fragment>
	<div class="p-2 h-full">
		{#each sections as section}
			<div class="card rounded-md shadow-sm p-2">
				<h4 class="font-bold">{section.title}</h4>
				<hr class="my-1" />
				<svelte:component this={section.component} {settings} />
			</div>
		{/each}
	</div>
</AppShell>

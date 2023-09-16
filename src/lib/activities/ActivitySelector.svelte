<script lang="ts">
	import DevopsModal from '$lib/azure_devops/DevopsModal.svelte';
	import type { Workitem } from '$lib/azure_devops/devops';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Alert, Input, Select, Toggle } from 'flowbite-svelte';

	export let value: number | string = '';
	export let useAzureDevops = false;
	export let workitems: Workitem[] = [];

	let hideAlert = true;
	let openModal = false;

	$: selectWorkitems = workitems.map((x) => {
		return { value: x.id, name: x.name };
	});

	const onToogleChange = () => {
		value = '';
		hideAlert = true;
		if (useAzureDevops) {
			invoke('get_workitems')
				.then((message) => {
					workitems = message as Workitem[];
					hideAlert = true;
				})
				.catch((error) => {
					if (error == 'authentication not valid') {
						openModal = true;
					} else {
						console.log(error);
						hideAlert = false;
					}
				});
		}
	};
</script>

<DevopsModal on:finished={onToogleChange} bind:open={openModal} />

<div class="h-10 mx-6 mt-6 mb-2 kimai-grid">
	{#if useAzureDevops}
		<Select items={selectWorkitems} bind:value />
	{:else}
		<Input type="text" placeholder="Activity" bind:value />
	{/if}
	<div class="self-center">
		<Toggle on:change={onToogleChange} bind:checked={useAzureDevops} color="blue"
			>Use Azure DevOps</Toggle
		>
	</div>
</div>

<Alert border color="red" class={`mx-6 ${hideAlert ? 'hidden' : ''}`}
	>DevOps Api ist not working</Alert
>

<style>
	.kimai-grid {
		display: grid;
		grid-template-columns: 1fr 8rem;
		gap: 1rem;
	}
</style>

<script lang="ts">
	import DevopsModal from '$lib/azure_devops/DevopsModal.svelte';
	import type { Workitem } from '$lib/azure_devops/devops';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Alert, Button, Input, Select, Toggle } from 'flowbite-svelte';

	export let value = '';
	export let useAzureDevops = false;

	let hideAlert = true;
	let openModal = false;
	let workitems: Workitem[] = [
		{ value: 'us', name: 'United States' },
		{ value: 'ca', name: 'Canada' },
		{ value: 'fr', name: 'France' }
	];

	const onToogleChange = () => {
		value = '';
		hideAlert = true;
		if (useAzureDevops) {
			invoke('get_workitems')
				.then((message) => {
					workitems = message as Workitem[];
				})
				.catch((error) => {
					if (error == 'Unauthorized') {
						//show Modal
					} else {
						console.log(error);
						hideAlert = false;
					}
				});
		}
	};
</script>

<DevopsModal bind:open={openModal} />

<div class="h-10 m-6 flex flex-row gap-4 justify-evenly">
	{#if useAzureDevops}
		<Select items={workitems} bind:value />
	{:else}
		<Input type="text" placeholder="Activity" bind:value />
	{/if}
	<Toggle on:change={onToogleChange} bind:checked={useAzureDevops} color="blue"
		>Use Azure DevOps</Toggle
	>
	<Button
		on:click={() => {
			openModal = true;
		}}
		>Modal
	</Button>
</div>

<Alert border color="red" class={`mx-6 ${hideAlert ? 'hidden' : ''}`}
	>DevOps Api ist not working</Alert
>

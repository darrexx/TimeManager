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

<div class="h-10 m-6 flex flex-row gap-4 justify-evenly">
	{#if useAzureDevops}
		<Select items={selectWorkitems} bind:value />
	{:else}
		<Input type="text" placeholder="Activity" bind:value />
	{/if}
	<Toggle on:change={onToogleChange} bind:checked={useAzureDevops} color="blue"
		>Use Azure DevOps</Toggle
	>
</div>

<Alert border color="red" class={`mx-6 ${hideAlert ? 'hidden' : ''}`}
	>DevOps Api ist not working</Alert
>

<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Alert, Select, Toggle } from 'flowbite-svelte';

	export let customer: number | string = '';
	export let project: number | string = '';
	export let activity: number | string = '';
	export let billable: boolean;
	export let useKimai = false;

	let customers: Customer[] = [];
	let projects: Project[] = [];
	let activities: Activity[] = [];
	let hideAlert = true;

	$: selectCustomers = customers.map((x) => {
		return { value: x.id, name: x.name };
	});

	$: selectProjects = projects.map((x) => {
		return { value: x.id, name: x.name };
	});

	$: selectActivities = activities.map((x) => {
		return { value: x.id, name: x.name };
	});

	const onToogleChange = () => {
		hideAlert = true;
		if (useKimai) {
			invoke('get_customers')
				.then((message) => {
					customers = message as Customer[];
					hideAlert = true;
				})
				.catch((error) => {
					console.log(error);
					hideAlert = false;
				});
		}
	};

	const onCustomerSelect = async () => {
		console.log(customer);
		projects = await invoke('get_projects', { customerId: customer });
	};

	const onProjectSelect = async () => {
		console.log(project);
		activities = await invoke('get_kimai_activities', { projectId: project });
	};
</script>

<div class="h-30 mx-6 mt-2 mb-6 kimai-grid">
	<Select
		items={selectCustomers}
		bind:value={customer}
		disabled={!useKimai}
		on:change={onCustomerSelect}
	/>

	<div class="self-center row-span-4">
		<Toggle on:change={onToogleChange} bind:checked={useKimai} color="blue">Track in Kimai</Toggle>
	</div>

	<Select
		type="text"
		placeholder="Project"
		items={selectProjects}
		bind:value={project}
		on:change={onProjectSelect}
		disabled={!useKimai || projects.length == 0}
	/>
	<Select
		type="text"
		placeholder="Activity"
		items={selectActivities}
		bind:value={activity}
		disabled={!useKimai || activities.length == 0}
	/>
	<div class="justify-self-end">
		<Toggle bind:checked={billable} color="red">Billable</Toggle>
	</div>
</div>

<Alert border color="red" class={`mx-6 ${hideAlert ? 'hidden' : ''}`}
	>Kimai Api ist not working</Alert
>

<style>
	.kimai-grid {
		display: grid;
		grid-template-columns: 1fr 8rem;
		grid-template-rows: repeat(3, minmax(0, 1fr));
		gap: 1rem;
	}
</style>

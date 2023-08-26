<script lang="ts">
	import ActivityTable from '$lib/activities/ActivityTable.svelte';
	import { createActivity, type CommandActivity } from '$lib/activities/activity';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Heading } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';

	let activities: CommandActivity[] = [];

	onMount(async () => {
		activities = await invoke('get_activities');
	});
</script>

<div class="bg-white dark:bg-gray-900 text-gray-600 dark:text-gray-400 py-2 min-h-screen">
	<div class="grid grid-cols-5 justify-items-center items-center">
		<a class="mr-auto" href="/"
			><button
				class="float-left ml-4 text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5"
				title="Back"
				><Icon class="self-center" name={'angle-left-outline'} />
			</button></a
		>
		<Heading
			tag="h1"
			class="mb-4 text-center dark:text-slate-200 col-span-3"
			customSize="text-4xl font-extrabold  md:text-5xl lg:text-5xl">Activity Overview</Heading
		>
		<div />
	</div>

	<ActivityTable activities={activities.map(createActivity)} />
</div>

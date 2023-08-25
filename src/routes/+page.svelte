<script lang="ts">
	import { DarkMode, Hr } from 'flowbite-svelte';
	import Timer from '$lib/Timer.svelte';
	import ActivitySelector from '$lib/activities/ActivitySelector.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { CommandActivity } from '$lib/activities/activity';
	import { createActivity } from '$lib/activities/activity';
	import ActivityHistory from '$lib/activities/ActivityHistory.svelte';
	import { onMount } from 'svelte';
	import type { Workitem } from '$lib/azure_devops/devops';
	import { Icon } from 'flowbite-svelte-icons';

	let currentActivity: number | string = '';
	let activities: CommandActivity[] = [];
	let useAzureDevops = false;
	let workitems: Workitem[];
	let popoutActive = false;

	onMount(async () => {
		activities = await invoke('get_activity_history');
	});

	const timerStarted = () => {
		if (useAzureDevops) {
			invoke('start_timer_with_workitem', {
				workitemName: workitems.find((x) => x.id == currentActivity)?.name,
				workitemId: currentActivity
			});
		} else {
			invoke('start_timer', { activityName: currentActivity });
		}
	};

	const timerStopped = () => {
		invoke('stop_timer');
		setTimeout(async () => {
			activities = await invoke('get_activity_history');
		}, 500);
	};

	const onPopout = () => {
		popoutActive = !popoutActive;
		invoke('toggle_popout', { active: popoutActive });
	};
</script>

<div class="bg-white dark:bg-gray-900 text-gray-600 dark:text-gray-400 pb-1 pt-1">
	<div class="flex flex-row gap-4 justify-between mx-4 mt-1">
		<DarkMode />
		<button
			title="Popout"
			class="text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5"
			on:click={onPopout}
			><Icon
				class="self-center"
				name={`${
					popoutActive ? 'arrow-left-to-bracket-outline' : 'arrow-right-to-bracket-outline'
				}`}
			/></button
		>
	</div>

	<Timer on:timerStarted={timerStarted} on:timerStopped={timerStopped} />

	<ActivitySelector bind:workitems bind:useAzureDevops bind:value={currentActivity} />

	<Hr classHr="m-8" />

	<!-- {#await activitiesPromise}
		<ListPlaceholder />
	{:then activities}
		<ActivityHistory activities={activities.map(createActivity)} />
	{/await} -->
	<ActivityHistory activities={activities.map(createActivity)} />
</div>

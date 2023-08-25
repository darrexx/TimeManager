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

	let currentActivity: number | string = '';
	let activities: CommandActivity[] = [];
	let useAzureDevops = false;
	let workitems: Workitem[];

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
</script>

<Timer on:timerStarted={timerStarted} on:timerStopped={timerStopped} />

<ActivitySelector bind:workitems bind:useAzureDevops bind:value={currentActivity} />

<Hr classHr="m-8" />

<!-- {#await activitiesPromise}
	<ListPlaceholder />
{:then activities}
	<ActivityHistory activities={activities.map(createActivity)} />
{/await} -->
<ActivityHistory activities={activities.map(createActivity)} />

<DarkMode />

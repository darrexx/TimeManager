<script lang="ts">
	import { DarkMode, Hr } from 'flowbite-svelte';
	import Timer from '$lib/Timer.svelte';
	import ActivitySelector from '$lib/activities/ActivitySelector.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { CommandActivity } from '$lib/activities/activity';
	import { createActivity } from '$lib/activities/activity';
	import ActivityHistory from '$lib/activities/ActivityHistory.svelte';
	import { onMount } from 'svelte';

	let currentActivity = '';
	let activities: CommandActivity[] = [];

	onMount(async () => {
		activities = await invoke('get_activity_history');
	});

	const timerStopped = () => {
		setTimeout(async () => {
			activities = await invoke('get_activity_history');
		}, 500);
	};
</script>

<Timer on:timerStopped={timerStopped} {currentActivity} />

<ActivitySelector bind:value={currentActivity} />

<Hr classHr="m-8" />

<!-- {#await activitiesPromise}
	<ListPlaceholder />
{:then activities}
	<ActivityHistory activities={activities.map(createActivity)} />
{/await} -->
<ActivityHistory activities={activities.map(createActivity)} />

<DarkMode />

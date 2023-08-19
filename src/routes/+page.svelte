<script lang="ts">
	import { DarkMode, Hr } from 'flowbite-svelte';
	import Timer from '$lib/Timer.svelte';
	import ActivitySelector from '$lib/activities/ActivitySelector.svelte';
	import ActivityHistory from '$lib/activities/ActivityHistory.svelte';

	let activities = ['1', '2', '3', '4', '5'];

	let currentActivity = '';

	const timerStopped = () => {
		console.log('asdfstopped');
		activities = [currentActivity, ...activities.slice(0, 4)];
	};
</script>

<Timer on:timerStopped={timerStopped} {currentActivity} />

<ActivitySelector bind:value={currentActivity} />

<Hr classHr="m-8" />

<ActivityHistory {activities} />

<DarkMode />

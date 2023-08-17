<script lang="ts">
	import { DarkMode, Hr } from 'flowbite-svelte';
	import Timer from '$lib/Timer.svelte';
	import ActivitySelector from '$lib/activities/ActivitySelector.svelte';
	import ActivityHistory from '$lib/activities/ActivityHistory.svelte';
</script>

<Timer />

<ActivitySelector />

<Hr classHr="m-8" />

<ActivityHistory />

<DarkMode />

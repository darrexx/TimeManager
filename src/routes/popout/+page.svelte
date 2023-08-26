<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { Heading } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration'; // import plugin

	let counter = 0;

	dayjs.extend(duration);

	onMount(async () => {
		await listen('timertick', (event) => {
			counter = event.payload as number;
		});
	});
</script>

<div data-tauri-drag-region class="bg-transparent">
	<Heading
		data-tauri-drag-region
		tag="h1"
		class="text-center"
		customSize="text-4xl font-extrabold  md:text-5xl lg:text-6xl"
		color="dark:text-white text-black"
		>{dayjs.duration(counter, 'seconds').format('HH:mm:ss')}</Heading
	>
</div>

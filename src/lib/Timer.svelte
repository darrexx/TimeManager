<script lang="ts">
	import { GradientButton, Heading } from 'flowbite-svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import { createEventDispatcher, onMount } from 'svelte';
	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration'; // import plugin

	let counter = 0;
	const dispatch = createEventDispatcher();

	dayjs.extend(duration);

	onMount(async () => {
		await listen('timertick', (event) => {
			counter = event.payload as number;
		});
	});

	const startTimer = () => {
		dispatch('timerStarted');
	};

	const resetTimer = () => {
		counter = 0;
		invoke('reset_timer');
	};

	const stopTimer = () => {
		dispatch('timerStopped');
	};
</script>

<Heading
	tag="h1"
	class="my-4 text-center"
	customSize="text-4xl font-extrabold  md:text-5xl lg:text-6xl"
	>{dayjs.duration(counter, 'seconds').format('HH:mm:ss')}</Heading
>

<div class="m-3 flex flex-row gap-5 justify-around">
	<GradientButton outline color="purpleToBlue" on:click={startTimer}>Start</GradientButton>
	<GradientButton outline color="purpleToBlue" on:click={stopTimer}>Stop</GradientButton>
	<GradientButton outline color="purpleToBlue" on:click={resetTimer}>Reset</GradientButton>
</div>

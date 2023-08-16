<script lang="ts">
	import Greet from '../lib/Greet.svelte';
	import { Button, DarkMode, Heading, Hr } from 'flowbite-svelte';
	import {invoke} from '@tauri-apps/api/tauri';
	import {listen} from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let counter = 0;
	let date = new Date();

	onMount(async () => {
		const unlisten = await listen("timertick", (event) => {
			console.log(event.payload)
			counter = event.payload as number;
		});
	})

</script>

<Button on:click={() => invoke("start_timer")}>Start</Button>

<Heading tag="h1" class="mb-4 text-center" customSize="text-4xl font-extrabold  md:text-5xl lg:text-6xl">{counter}</Heading>

<Hr classHr="my-8" />

<DarkMode></DarkMode>
<Heading tag="h1" class="mb-4 text-center" customSize="text-4xl font-extrabold  md:text-5xl lg:text-6xl">{date.toLocaleTimeString()}</Heading>
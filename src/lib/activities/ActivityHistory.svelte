<script lang="ts">
	import { Heading, Timeline, TimelineItem } from 'flowbite-svelte';
	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration'; // import plugin

	dayjs.extend(duration);

	export let activities: string[] = ['1', '2', '3', '4', '5'];
</script>

<Heading
	tag="h2"
	class="my-4 text-center dark:text-slate-200"
	customSize="text-3xl font-extrabold  md:text-5xl lg:text-5xl">Activities</Heading
>
<Timeline class="mx-4">
	{#each activities as activity}
		<TimelineItem
			title={`Title${activity}`}
			date={`🕒${dayjs.duration(Math.random() * 20000, 'seconds').format('HH:mm:ss')}`}
		>
			<!-- todo mit icon https://flowbite-svelte.com/docs/components/timeline#Vertical_Timeline -->
			<p class="text-base font-normal text-gray-500 dark:text-gray-400">
				Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor
				invidunt ut labore et dolore magna aliquyam.
			</p>
		</TimelineItem>
	{/each}
</Timeline>

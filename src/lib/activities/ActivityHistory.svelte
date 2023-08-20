<script lang="ts">
	import { Heading, Timeline, TimelineItem } from 'flowbite-svelte';
	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration'; // import plugin
	import relativeTime from 'dayjs/plugin/relativeTime';
	import type { Activity } from './activity';

	dayjs.extend(duration);
	dayjs.extend(relativeTime);

	export let activities: Activity[];
</script>

<Heading
	tag="h2"
	class="my-4 text-center dark:text-slate-200"
	customSize="text-3xl font-extrabold  md:text-5xl lg:text-5xl">Activities</Heading
>
<Timeline class="mx-4">
	{#each activities as activity}
		<!-- TODO display more than 24 Hours -->
		<TimelineItem title={activity.name} date={`🕒${activity.timeSpent.format('HH:mm:ss')}`}>
			<!-- todo mit icon https://flowbite-svelte.com/docs/components/timeline#Vertical_Timeline -->
			<p class="text-base font-normal text-gray-500 dark:text-gray-400">
				Last worked on: {dayjs().to(activity.lastModified)} <br />
				Work started: {dayjs().to(activity.createdAt)}
			</p>
		</TimelineItem>
	{/each}
</Timeline>

<script lang="ts">
	import { Timeline, TimelineItem } from 'flowbite-svelte';
	import dayjs from 'dayjs';
	import type { Activity, ActivityTime } from './activity';

	export let activity_times: ActivityTime[];
	export let activities: Activity[];
</script>

<Timeline class="mx-4">
	{#each activity_times as activity_time}
		<TimelineItem
			title={activities.find((x) => x.id == activity_time.activity_id)?.name ?? ''}
			date={`🕒${activity_time.start_time.format('DD.MM. HH:mm:ss')}`}
		>
			<p class="text-base font-normal text-gray-500 dark:text-gray-400">
				Work started: {activity_time.start_time.format('DD.MM. HH:mm:ss')} <br />
				Work ended: {activity_time.end_time.format('DD.MM. HH:mm:ss')}
			</p>
		</TimelineItem>
	{/each}
</Timeline>

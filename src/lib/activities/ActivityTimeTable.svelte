<script lang="ts">
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration';
	import type { Activity, ActivityTime } from './activity';

	dayjs.extend(duration);

	export let activityTimes: ActivityTime[] = [];
	export let activities: Activity[] = [];
</script>

<Table hoverable divClass="m-4 overflow-x-auto">
	<TableHead>
		<TableHeadCell>Id</TableHeadCell>
		<TableHeadCell>Start Time</TableHeadCell>
		<TableHeadCell>End Time</TableHeadCell>
		<TableHeadCell>Activity Name</TableHeadCell>
	</TableHead>
	<TableBody tableBodyClass="divide-y">
		{#each activityTimes as activityTime}
			<TableBodyRow>
				<TableBodyCell>{activityTime.id}</TableBodyCell>
				<TableBodyCell>{activityTime.start_time.format('DD.MM.YYYY HH:mm:ss')}</TableBodyCell>
				<TableBodyCell>{activityTime.end_time.format('DD.MM.YYYY HH:mm:ss')}</TableBodyCell>
				<TableBodyCell
					>{activities.find((x) => x.id == activityTime.activity_id)?.name ?? ''}</TableBodyCell
				>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>

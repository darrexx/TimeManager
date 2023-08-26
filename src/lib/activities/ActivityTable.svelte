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
	import type { Activity } from './activity';

	dayjs.extend(duration);

	export let activities: Activity[] = [];
</script>

<Table hoverable divClass="m-4 overflow-x-auto">
	<TableHead>
		<!-- <TableHeadCell>Id</TableHeadCell> -->
		<TableHeadCell>Name</TableHeadCell>
		<TableHeadCell>Duration</TableHeadCell>
		<TableHeadCell>Last Modified</TableHeadCell>
		<TableHeadCell>Created</TableHeadCell>
		<TableHeadCell>WorkitemId</TableHeadCell>
	</TableHead>
	<TableBody tableBodyClass="divide-y">
		{#each activities as activity}
			<TableBodyRow>
				<!-- <TableBodyCell>{activity.id}</TableBodyCell> -->
				<TableBodyCell>{activity.name}</TableBodyCell>
				<TableBodyCell>{activity.timeSpent.format('HH:mm:ss')}</TableBodyCell>
				<TableBodyCell>{activity.lastModified.format('DD.MM.YYYY H:mm')}</TableBodyCell>
				<TableBodyCell>{activity.createdAt.format('DD.MM.YYYY H:mm')}</TableBodyCell>
				<TableBodyCell>{activity.workitem_id ?? ''}</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>

import dayjs, { Dayjs } from 'dayjs';
import type duration from 'dayjs/plugin/duration'; // import plugin

export interface Activity {
	id: number;
	name: string;
	timeSpent: duration.Duration;
	lastModified: Dayjs;
	createdAt: Dayjs;
}

export interface CommandActivity {
	created_at: number;
	duration: number;
	id: number;
	last_modified: number;
	name: string;
}

export function createActivity(commandActivity: CommandActivity): Activity {
	return {
		createdAt: dayjs(commandActivity.created_at),
		id: commandActivity.id,
		lastModified: dayjs(commandActivity.last_modified),
		name: commandActivity.name,
		timeSpent: dayjs.duration(dayjs(commandActivity.duration).diff(dayjs.unix(0)))
	};
}

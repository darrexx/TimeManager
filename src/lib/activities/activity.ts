import dayjs, { Dayjs } from 'dayjs';
import type duration from 'dayjs/plugin/duration'; // import plugin

export interface Activity {
	id: number;
	name: string;
	timeSpent: duration.Duration;
	lastModified: Dayjs;
	createdAt: Dayjs;
	workitem_id: number;
}

export interface CommandActivity {
	created_at: number;
	duration: number;
	id: number;
	last_modified: number;
	name: string;
	workitem_id: number;
}

export function createActivity(commandActivity: CommandActivity): Activity {
	return {
		createdAt: dayjs(commandActivity.created_at),
		id: commandActivity.id,
		lastModified: dayjs(commandActivity.last_modified),
		name: commandActivity.name,
		timeSpent: dayjs.duration(dayjs(commandActivity.duration).diff(dayjs.unix(0))),
		workitem_id: commandActivity.workitem_id
	};
}

export interface ActivityTime {
	id: number;
	activity_id: number;
	start_time: Dayjs;
	end_time: Dayjs;
}

export interface CommandActivityTime {
	id: number;
	activity_id: number;
	start_time: number;
	end_time: number;
}

export function createActivityTime(commandActivityTime: CommandActivityTime): ActivityTime {
	return {
		id: commandActivityTime.id,
		activity_id: commandActivityTime.activity_id,
		start_time: dayjs(commandActivityTime.start_time),
		end_time: dayjs(commandActivityTime.end_time)
	};
}

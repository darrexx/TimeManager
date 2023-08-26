import { invoke } from '@tauri-apps/api/tauri';

export interface Workitem {
	id: number;
	name: string;
}

export enum ModalState {
	GetProject,
	GetTeams,
	Finish
}

export async function getProjects(
	url: string,
	user: string,
	pat: string,
	organization: string
): Promise<string[]> {
	const base64 = btoa(`${user}:${pat}`);

	const response = await fetch(`https://${url}/${organization}/_apis/projects?api-version=7.0`, {
		headers: { Authorization: `Basic ${base64}` }
	});
	if (!response.ok) {
		return [];
	}
	const json = await response.json();
	return json.value.map((x: { name: string }) => x.name);
}

export async function getTeams(
	url: string,
	user: string,
	pat: string,
	organization: string,
	project: string
): Promise<string[]> {
	const base64 = btoa(`${user}:${pat}`);

	const response = await fetch(
		`https://${url}/${organization}/_apis/projects/${project}/teams?api-version=7.0`,
		{
			headers: { Authorization: `Basic ${base64}` }
		}
	);
	if (!response.ok) {
		return [];
	}
	const json = await response.json();
	return json.value.map((x: { name: string }) => x.name);
}

export function saveDevopsConfig(
	url: string,
	user: string,
	pat: string,
	organization: string,
	project: string,
	team: string
) {
	invoke('save_devops_config', {
		url: url,
		user: user,
		pat: pat,
		organization: organization,
		project: project,
		team: team
	});
}

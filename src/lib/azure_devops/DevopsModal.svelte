<script lang="ts">
	import { Button, Input, Label, Modal, Select } from 'flowbite-svelte';
	import { ModalState, getProjects, getTeams, saveDevopsConfig } from './devops';
	import { createEventDispatcher } from 'svelte';

	export let open = false;
	let state = ModalState.GetProject;
	let server = '';
	let user = '';
	let pat = '';
	let organization = '';
	let selectedProject: string;
	let selectedTeam: string;

	let projects: { name: string; value: string }[] = [];
	let teams: { name: string; value: string }[] = [];

	const dispatch = createEventDispatcher();

	const handleClick = async () => {
		switch (state) {
			case ModalState.GetProject:
				projects = (await getProjects(server, user, pat, organization)).map((x) => {
					return { name: x, value: x };
				});
				if (projects.length != 0) {
					state = ModalState.GetTeams;
				}
				break;
			case ModalState.GetTeams:
				teams = (await getTeams(server, user, pat, organization, selectedProject)).map((x) => {
					return { name: x, value: x };
				});
				if (teams.length != 0) {
					state = ModalState.Finish;
				}
				break;
			case ModalState.Finish:
				saveDevopsConfig(server, user, pat, organization, selectedProject, selectedTeam);
				open = false;
				dispatch('finished');
				break;
		}
	};
</script>

<Modal bind:open size="sm">
	<h2 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Azure Devops Configuration</h2>
	<div class="grid grid-cols-2 gap-6">
		<div class="col-span-2">
			<Label for="devops_server" class="mb-2">Azure Devops Server</Label>
			<Input
				placeholder="dev.azure.com"
				id="devops_server"
				required
				bind:value={server}
				disabled={state != ModalState.GetProject}
			/>
		</div>
		<div class="col-span-2">
			<Label for="devops_organization" class="mb-2">Organization</Label>
			<Input
				placeholder="Organization"
				id="devops_organization"
				required
				bind:value={organization}
				disabled={state != ModalState.GetProject}
			/>
		</div>
		<div>
			<Label for="devops_user" class="mb-2">User</Label>
			<Input
				placeholder="user"
				id="devops_user"
				required
				bind:value={user}
				disabled={state != ModalState.GetProject}
			/>
		</div>
		<div>
			<Label for="devops_pat" class="mb-2">PAT</Label>
			<Input
				placeholder="PAT"
				id="devops_pat"
				required
				bind:value={pat}
				disabled={state != ModalState.GetProject}
			/>
		</div>
		{#if projects.length != 0}
			<div class="col-span-2">
				<Label for="devops_projects" class="mb-2">Project</Label>
				<Select
					id="devops_projects"
					items={projects}
					bind:value={selectedProject}
					placeholder="Select project..."
					disabled={state != ModalState.GetTeams}
				/>
			</div>
		{/if}
		{#if teams.length != 0}
			<div class="col-span-2">
				<Label for="devops_teams" class="mb-2">Teams</Label>
				<Select
					id="devops_teams"
					items={teams}
					bind:value={selectedTeam}
					placeholder="Select teams..."
				/>
			</div>
		{/if}
		<Button on:click={handleClick} class="col-start-2">Next</Button>
	</div>
</Modal>

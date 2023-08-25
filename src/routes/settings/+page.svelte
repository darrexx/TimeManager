<script>
	// @ts-nocheck

	import { invoke } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { Alert, Heading, Label, Input, Button, Span } from 'flowbite-svelte';
	import { onMount } from 'svelte';

	let configs = {
		devops: {
			name: 'string',
			plz: 'asdf'
		}
	};

	onMount(async () => {
		configs = await invoke('get_config');
	});

	const onSave = async () => {
		await invoke('set_config', { config: configs });
		let settingsWindow = WebviewWindow.getByLabel('settings');
		settingsWindow.close();
	};
</script>

<div class="bg-white dark:bg-gray-900 text-gray-600 dark:text-gray-400 pt-4 pb-14 min-h-screen">
	<Alert class="mx-4 mb-4" color="red" border
		>The Settings will <Span underline decorationClass="decoration-red-500 decoration-double"
			>only</Span
		> be applied after a Restart</Alert
	>

	<Heading
		tag="h1"
		class="my-4 text-center dark:text-slate-200"
		customSize="text-4xl font-extrabold  md:text-5xl lg:text-5xl">Settings</Heading
	>

	{#each Object.entries(configs) as config}
		<Heading
			tag="h2"
			class="my-4 text-center dark:text-slate-200"
			customSize="text-3xl font-extrabold  md:text-4xl lg:text-4xl">{config[0]}</Heading
		>
		<div class="flex flex-col m-4 gap-4">
			{#each Object.entries(config[1]) as setting}
				<div>
					<Label for={setting[0]} class="mb-2">{setting[0]}</Label>
					<Input id={setting[0]} bind:value={config[1][setting[0]]} />
				</div>
			{/each}
		</div>
	{/each}

	<Button class="mb-4 mx-4 float-right" on:click={onSave}>Save Settings</Button>
</div>

<script lang="ts">
	import { Button, Input } from 'flowbite-svelte';

	export let value = '';
</script>

<div class="m-6 flex flex-row gap-4 justify-evenly">
	<Input type="text" placeholder="Activity" bind:value />
</div>

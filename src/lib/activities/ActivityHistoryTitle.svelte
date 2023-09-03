<script lang="ts">
	import { Heading } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
</script>

<div class="grid grid-cols-3 justify-items-center items-center">
	<a class="mr-auto" href="/activityoverview"
		><button
			class="float-right ml-4 text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5"
			title="More..."
		>
			<Icon size="lg" class="self-center" name={'book-outline'} />
		</button></a
	>
	<Heading
		tag="h2"
		class="mb-4 text-center dark:text-slate-200"
		customSize="text-3xl font-extrabold  md:text-5xl lg:text-5xl">Activities</Heading
	>
	<a class="ml-auto" href="/activitytimeoverview"
		><button
			class="float-right mr-4 text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5"
			title="More..."
		>
			<Icon size="lg" class="self-center" name={'clock-outline'} />
		</button></a
	>
</div>

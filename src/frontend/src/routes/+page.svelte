<script lang="ts">
	import PageSelector from '$components/PageSelector.svelte';

	import AbsenceCard from '$components/AbsenceCard.svelte';
	import type { components } from '$lib/api/schema';
	import AddAbsenceDrawer from '$components/AddAbsenceDrawer.svelte';
	import { title } from '$lib/stores/title.store';
	import { formatItalianDate } from '$lib/utils/dates';
	import { Toaster } from '$lib/components/ui/sonner/index.js';

	const { data }: { data: { date: string | null; absences: components['schemas']['Absence'][] } } =
		$props();

	const formattedDate = $derived(data.date ? formatItalianDate(data.date as string) : 'oggi');

	$effect(() => {
		title.set('Assenze per ' + formattedDate);
	});

	let openAbsenceDrawer = $state(false);
</script>

<svelte:head>
	<title>Presenze Meucci</title>
</svelte:head>

<Toaster richColors position="bottom-center" />

<main class="flex justify-center">
	<div class="w-full max-w-3xl">
		<PageSelector
			class="mx-3 my-4 md:mx-0"
			disabled={openAbsenceDrawer}
			date={data.date}
			{formattedDate}
		/>
		<div class="flex w-full flex-col items-center justify-center gap-4">
			{#each data.absences as absence (absence.absent_teacher)}
				<AbsenceCard {absence} />
			{/each}
		</div>

		<AddAbsenceDrawer bind:open={openAbsenceDrawer} date={data.date} {formattedDate} />
	</div>
</main>

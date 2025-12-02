<script lang="ts">
	import PageSelector from '$components/PageSelector.svelte';

	import AbsenceCard from '$components/AbsenceCard.svelte';
	import type { components } from '$lib/api/schema';
	import AddAbsenceDrawer from '$components/AddAbsenceDrawer.svelte';

	const { data } = $props<components['schemas']['Absence'][]>();
</script>

<main class="flex justify-center">
	<div class="w-full max-w-[600px]">
		<PageSelector class="mx-20 my-4 md:mx-0" />
		<div class="flex w-full flex-col items-center justify-center gap-4">
			{#each data.absences as absence (absence.id)}
				<AbsenceCard {absence} class="min-w-full" />
			{/each}
		</div>

		<AddAbsenceDrawer />
	</div>
</main>

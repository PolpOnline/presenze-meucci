<script lang="ts">
	import { Button } from '$components/ui/button/index.js';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer/index.js';
	import { Input } from '$components/ui/input/index.js';
	import PageSelector from '$components/PageSelector.svelte';
	import LucidePlus from '~icons/lucide/plus';
    import ProfessorPicker from "$components/ProfessorPicker.svelte";
//    import AbsenceCard from "$components/AbsenceCard.svelte";

	const { data } = $props();

	console.log(data.absences);

</script>

<main class="flex justify-center">
	<div class="w-full max-w-[600px]">


		<PageSelector class="my-4 mx-20 md:mx-0" />
        <!--
               {#each data.absences as absence (absence.id)}
                  <AbsenceCard data={absence} />
                {/each}
        -->
        <Drawer.Trigger>
            <Button variant="secondary" size="icon" class="fixed rounded-2 right-5 bottom-5 size-12">
                <LucidePlus class="size-6" />
            </Button>
        </Drawer.Trigger>
        <Drawer.Content class="rounded-t-2xl border-t bg-background p-6 shadow-xl flex flex-col gap-4 items-center justify-center">
            <Drawer.Header class="mt-20">
                <Drawer.Title class="text-[30px] font-semibold">Aggiungi assenza professore</Drawer.Title>
            </Drawer.Header>

            <ProfessorPicker/>
            <Input type="number" placeholder="Dalle ore" class="max-w-xl" />
            <Input type="number" placeholder="Alle ore" class="max-w-xl" />

            <Drawer.Footer class="flex flex-row justify-center mb-20 w-full]">
                <Button class="max-w-xl w-full">Add</Button>
            </Drawer.Footer>
        </Drawer.Content>
    </div>
</main>

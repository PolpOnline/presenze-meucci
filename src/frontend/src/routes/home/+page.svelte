<script lang="ts">
    import DatePicker from '$lib/components/DatePicker.svelte';
    import type {DateValue} from '@internationalized/date';
    import {Button} from '$lib/components/ui/button';
    import SelectProf from "$lib/components/SelectProf.svelte";

    let date: DateValue | null = null;
</script>

<main>
    <h1 class="mt-5 text-center text-3xl font-bold">Supplenze</h1>
    <div
            class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col content-center space-y-4 pb-1 align-middle"
    >
        <DatePicker bind:value={date}/>

        <SelectProf/>

        <Button variant="secondary" href="">Cerca sostituzioni</Button>
    </div>
</main>

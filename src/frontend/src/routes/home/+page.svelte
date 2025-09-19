<script lang="ts">
    import DatePicker from '$lib/components/DatePicker.svelte';
    import type {DateValue} from '@internationalized/date';
    import {Button} from '$lib/components/ui/button';
    import SelectProf from "$lib/components/SelectProf.svelte";
    import * as Card from "$lib/components/ui/card/index.js";

    let date: DateValue | null = null;
</script>

<main>

    <div
            class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col content-center space-y-4 pb-1 align-middle"
    >
        <Card.Root>
            <Card.Header>
                <Card.Title>
                    <h1 class="mt-5 text-center text-3xl font-bold">Supplenze</h1>
                </Card.Title>
            </Card.Header>
            <Card.Content>
                <div class="flex flex-col items-center gap-2">
                    <DatePicker bind:value={date}/>

                    <SelectProf/>

                    <Button variant="secondary" href="">Cerca sostituzioni</Button>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
</main>

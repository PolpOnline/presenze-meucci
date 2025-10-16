<script lang="ts">
    import DatePicker from '$lib/components/DatePicker.svelte';
    import type {DateValue} from '@internationalized/date';
    import {Button} from '$lib/components/ui/button';
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Select from "$lib/components/ui/select";

    let { data } = $props();
    let value = $state("");

    const triggerContent = $derived(
        data.professors.find((p) => p.id.toString() === value)?.full_name ?? "Seleziona un professore"
    )
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

                    <Select.Root type="single" name="profselector">
                        <Select.Trigger class="w-[280px]">
                            {triggerContent}
                        </Select.Trigger>
                        <Select.Content>
                            <Select.Group>
                                <Select.Label>Professori</Select.Label>
                                {#each data.professors as prof (prof.id)}
                                    <Select.Item
                                            value={prof.id.toString()}
                                            label={prof.full_name}
                                    >
                                        {prof.full_name}
                                    </Select.Item>
                                {/each}
                            </Select.Group>
                        </Select.Content>
                    </Select.Root>

                    <Button variant="secondary" href="">Cerca sostituzioni</Button>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
</main>

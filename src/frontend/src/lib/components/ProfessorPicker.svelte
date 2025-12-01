<script lang="ts">
    import CheckIcon from "@lucide/svelte/icons/check";
    import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
    import { tick } from "svelte";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { cn } from "$lib/utils.js";
    import {client} from "$lib/api/api";


    let open = $state(false);
    let value = $state("");
    let triggerRef = $state<HTMLButtonElement>(null!);

    let professors: Array<{full_name:string, id:number}> = [];

    const selectedValue = $derived(
        professors.find((f: {full_name:string, id:number}) => f.id === Number(value))?.id,
    );

    function closeAndFocusTrigger() {
        open = false;
        tick().then(() => {
            triggerRef.focus();
        });
    }

    async function getProfessors() {
        const res = await client.GET("/teachers/can_be_absent");

        professors = res.data ?? [];

        return professors;
    }

</script>

<Popover.Root bind:open>
    <Popover.Trigger bind:ref={triggerRef}>
        {#snippet child({ props })}
            <Button
                    variant="outline"
                    class="w-[200px] justify-between"
                    {...props}
                    role="combobox"
                    aria-expanded={open}
            >
                {selectedValue || "Seleziona un professore"}
                <ChevronsUpDownIcon class="ms-2 size-4 shrink-0 opacity-50" />
            </Button>
        {/snippet}
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0">
        <Command.Root>
            <Command.Input placeholder="Cerca professore..." />
            <Command.List>
                <Command.Empty>Nessun professore trovato!</Command.Empty>
                <Command.Group>

                    <!-- TODO: Fix this-->

                    {#await getProfessors()}

                    {:then professors}
                        {#each professors as professor (professor.id)}
                        <Command.Item
                            value={professor.id.toString()}
                            onSelect={() => {
                            value = professor.id.toString();
                            closeAndFocusTrigger();
                             }}
                        >
                        <CheckIcon
                            class={cn("me-2 size-4",
                            value !== professor.id.toString() && "text-transparent"
                            )}
                        />
                            {professor.full_name}
                        </Command.Item>
                        {/each}
                    {:catch error}
                        <div class="text-destructive-foreground">
                            <p>Errore: {error.message}</p>
                        </div>
                    {/await}
                </Command.Group>
            </Command.List>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
<script lang="ts">
    import CheckIcon from "@lucide/svelte/icons/check";
    import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
    import { tick } from "svelte";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { cn } from "$lib/utils.js";

    const professors = $props<{full_name:string, id:number}[]>();

    let open = $state(false);
    let value = $state("");
    let triggerRef = $state<HTMLButtonElement>(null!);

    const selectedValue = $derived(
        professors.find((f: {full_name:string, id:number}) => f.id === Number(value))?.label
    );

    function closeAndFocusTrigger() {
        open = false;
        tick().then(() => {
            triggerRef.focus();
        });
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
                    {#each professors as professor}
                        <Command.Item
                                value={professor.id}
                                onSelect={() => {
        value = professor.id;
        closeAndFocusTrigger();
       }}
                        >
                            <CheckIcon
                                    class={cn(
         "me-2 size-4",
         value !== professor.id && "text-transparent"
        )}
                            />
                            {professor.full_name}
                        </Command.Item>
                    {/each}
                </Command.Group>
            </Command.List>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
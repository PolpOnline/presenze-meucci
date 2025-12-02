<script lang="ts">
    import {Badge} from '$components/ui/badge';
    import type {components} from '$lib/api/schema';
    import LucideDoorClosed from '~icons/lucide/door-closed';
    import LucideUsers from '~icons/lucide/users';


    let { absence }: { absence: components['schemas']['Absence'] } = $props();
</script>

<div class="rounded-xl border p-4 w-full">
    <div class="text-lg tracking-wide font-bold mb-0.5 ml-1">
        {absence.absent_teacher}
    </div>

    {#each absence.classes as hour (hour.id)}
        <div class="text-lg font-medium text-muted-foreground ml-2">{hour.time}</div>
        <div class="p-3 shadow-sm border rounded-xl">
            <div class="flex flex-col">
                {#if hour.absent_status === 'uncovered'}
                    <div class="text-sm font-semibold text-destructive">SCOPERTO</div>
                {:else}
                    <div class="text-sm">{hour.substitute_teacher}</div>
                {/if}

                <div class="mt-2 flex gap-2">
                    <Badge variant="secondary">
                        <LucideDoorClosed class="mr-1 size-4" />
                        {hour.group}
                    </Badge>
                    <Badge variant="secondary">
                        <LucideUsers class="mr-1 size-4" />
                        {hour.room}
                    </Badge>
                </div>
            </div>
        </div>
    {/each}
</div>
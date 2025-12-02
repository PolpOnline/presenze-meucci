<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import MoreHorizontal from "@lucide/svelte/icons/more-horizontal";
    import {Badge} from "$components/ui/badge";

    let { data } = $props<{
        absent_teacher: string;
        classes: Array<{
            absent_status: "uncovered" | "substituteFound" | "classCanceled" | "classDelayed";
            className: string;
            group: string | null;
            id: number;
            room: string | null;
            substitute_teacher: string | null;
            time: string;
        }>;
    }>();

</script>

<Card.Root class="w-full p-10 space-y-4">
    <Card.Header>
        <Card.Title class="text-xl font-semibold">{data.absent_teacher}</Card.Title>
    </Card.Header>

    <Card.Content>
        {#each data.classes as hour (hour.id)}
            <div class="border rounded-xl p-3 space-y-2 shadow-sm">

                <div class="text-lg font-medium">{hour.time}</div>

                <div class="flex items-start justify-between">
                    <div class="flex flex-col">

                        {#if hour.absent_status === "covered"}
                            <div class="text-sm">{hour.substitute_teacher}</div>
                        {:else}
                            <div class="text-sm font-semibold text-red-600">SCOPERTO</div>
                        {/if}

                        <div class="flex gap-2 mt-2">
                            <Badge variant="outline">{hour.group}</Badge>
                            <Badge variant="outline">{hour.room}</Badge>
                        </div>
                    </div>

                    <MoreHorizontal class="w-5 h-5 opacity-70"/>
                </div>

            </div>
        {/each}
    </Card.Content>
</Card.Root>

<script lang="ts">
	import { Badge } from '$components/ui/badge';
	import type { components } from '$lib/api/schema';
	import LucideDoorClosed from '~icons/lucide/door-closed';
	import LucideUsers from '~icons/lucide/users';
	import LucidePencil from '~icons/lucide/pencil';
	import LucideTrash2 from '~icons/lucide/trash-2';
	import ButtonTooltip from '$components/ButtonTooltip.svelte';

	let { absence }: { absence: components['schemas']['Absence'] } = $props();
</script>

<div class="w-full rounded-xl border p-4">
	<div class="mb-3 ml-1 text-lg font-bold tracking-wide">
		{absence.absent_teacher}
	</div>

	<div class="flex flex-col space-y-2">
		{#each absence.classes as hour (hour.id)}
			{@const formattedTime = hour.time.split(':', 2).slice(0, 2).join(':')}
			<div>
				<div class="ml-2 text-lg font-medium text-muted-foreground">{formattedTime}</div>

				<div class="grid grid-cols-12 rounded-xl border p-3 shadow-sm">
					<div class="col-span-10">
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
					<div class="col-span-2 flex items-center justify-end space-x-2">
						<ButtonTooltip size="icon" label="Modifica assenza" variant="outline">
							<LucidePencil class="size-4" />
						</ButtonTooltip>
						<ButtonTooltip
							size="icon"
							class="text-destructive"
							label="Elimina assenza"
							variant="outline"
						>
							<LucideTrash2 class="size-4 text-destructive" />
						</ButtonTooltip>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>

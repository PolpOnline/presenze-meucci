<script lang="ts">
	import { Badge } from '$components/ui/badge';
	import type { components } from '$lib/api/schema';
	import LucideDoorClosed from '~icons/lucide/door-closed';
	import LucideUsers from '~icons/lucide/users';

	let { absence }: { absence: components['schemas']['Absence'] } = $props();
</script>

<div class="w-full rounded-xl border p-4">
	<div class="mb-0.5 ml-1 text-lg font-bold tracking-wide">
		{absence.absent_teacher}
	</div>

	{#each absence.classes as hour (hour.id)}
		<div class="ml-2 text-lg font-medium text-muted-foreground">{hour.time}</div>
		<div class="rounded-xl border p-3 shadow-sm">
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

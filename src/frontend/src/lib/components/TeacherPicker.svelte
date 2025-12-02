<script lang="ts">
	import CheckIcon from '~icons/lucide/check';
	import ChevronsUpDownIcon from '~icons/lucide/chevrons-up-down';
	import { tick } from 'svelte';
	// noinspection ES6UnusedImports
	import * as Command from '$lib/components/ui/command/index.js';
	// noinspection ES6UnusedImports
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { cn } from '$lib/utils.js';
	import { client } from '$lib/api/api';

	let {
		value = $bindable()
	}: {
		value: number | null;
	} = $props();

	let open = $state(false);
	let triggerRef = $state<HTMLButtonElement>(null!);

	let selectedValue = $state<string | null>(null);

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}

	async function getTeachers() {
		const res = await client.GET('/teachers/can_be_absent');

		return res.data ?? [];
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger bind:ref={triggerRef} class="w-full max-w-lg justify-between">
		{#snippet child({ props })}
			<Button variant="outline" class="w-full" {...props} role="combobox" aria-expanded={open}>
				{selectedValue || 'Seleziona un professore'}
				<ChevronsUpDownIcon class="ms-2 size-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-(--bits-popover-anchor-width) p-0">
		<Command.Root>
			<Command.Input placeholder="Cerca professore..." />
			<Command.List>
				<Command.Empty>Nessun professore trovato!</Command.Empty>
				<Command.Group>
					{#await getTeachers()}
						<p class="p-2">Loading…</p>
					{:then teachers}
						{#each teachers as teacher (teacher.id)}
							<Command.Item
								value={teacher.id.toString()}
								onSelect={() => {
									value = teacher.id;
									selectedValue = teacher.full_name;
									closeAndFocusTrigger();
								}}
							>
								<CheckIcon class={cn('me-2 size-4', value !== teacher.id && 'text-transparent')} />
								{teacher.full_name}
							</Command.Item>
						{/each}
					{:catch error}
						<p class="text-destructive-foreground p-2">
							Errore: {error.message}
						</p>
					{/await}
				</Command.Group>
			</Command.List>
		</Command.Root>
	</Popover.Content>
</Popover.Root>

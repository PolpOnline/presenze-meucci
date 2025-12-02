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
	import type { components } from '$lib/api/schema';

	let open = $state(false);
	let value = $state('');
	let triggerRef = $state<HTMLButtonElement>(null!);

	let professors = $state<components['schemas']['CanBeAbsentTeacher'][]>([]);

	const selectedValue = $derived(professors.find((f) => f.id === Number(value))?.full_name);

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}

	async function getProfessors() {
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
					{#await getProfessors()}
						<p class="p-2">Loading…</p>
					{:then professors}
						{#each professors as professor (professor.id)}
							<Command.Item
								value={professor.full_name}
								onSelect={() => {
									value = professor.id.toString();
									closeAndFocusTrigger();
								}}
							>
								<CheckIcon
									class={cn('me-2 size-4', value !== professor.id.toString() && 'text-transparent')}
								/>
								{professor.full_name}
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

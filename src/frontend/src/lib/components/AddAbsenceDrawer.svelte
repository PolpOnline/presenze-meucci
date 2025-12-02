<script>
	import LucidePlus from '~icons/lucide/plus';
	import { Input } from '$components/ui/input/index';
	// noinspection ES6UnusedImports
	import * as Drawer from '$components/ui/drawer/index';
	import ProfessorPicker from '$components/ProfessorPicker.svelte';
	import { Button } from '$components/ui/button/index';
	import { cn } from '$lib/utils';

	let hideTrigger = $state(false);
</script>

<Drawer.Root
	onOpenChange={(o) => {
		if (o) hideTrigger = o;
	}}
	onOpenChangeComplete={(o) => {
		if (!o) hideTrigger = o;
	}}
>
	<Drawer.Trigger>
		<Button
			variant="secondary"
			size="icon"
			class={cn(hideTrigger ? 'hidden' : '', 'rounded-2 fixed right-5 bottom-5 size-12')}
		>
			<LucidePlus class="size-6" />
		</Button>
	</Drawer.Trigger>
	<Drawer.Content
		class="flex flex-col items-center justify-center gap-4 rounded-t-2xl border-t bg-background p-6 shadow-xl"
	>
		<Drawer.Header>
			<Drawer.Title class="text-center text-3xl font-semibold">
                Aggiungi assenza professore
            </Drawer.Title>
		</Drawer.Header>

		<ProfessorPicker />

		<Input type="date" placeholder="Del giorno" class="max-w-lg" id="date" />

		<Input type="time" placeholder="Dalle ore" class="max-w-lg" id="begin_time" />

		<Input type="time" placeholder="Alle ore" class="max-w-lg" id="end_time" />

		<Drawer.Footer class="flex w-full flex-row justify-center">
			<Button class="w-full max-w-lg">Add</Button>
		</Drawer.Footer>
	</Drawer.Content>
</Drawer.Root>

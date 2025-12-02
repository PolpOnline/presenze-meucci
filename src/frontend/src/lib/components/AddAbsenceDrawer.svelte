<script lang="ts">
	import LucidePlus from '~icons/lucide/plus';
	import { Input } from '$components/ui/input/index';
	// noinspection ES6UnusedImports
	import * as Drawer from '$components/ui/drawer/index';
	import TeacherPicker from '$components/TeacherPicker.svelte';
	import { Button } from '$components/ui/button/index';
	import { cn } from '$lib/utils';
	import { client } from '$lib/api/api';
    import {invalidateAll} from "$app/navigation";

	let open = $state(false);

	let hideTrigger = $state(false);

	let absent_teacher_id: number | null = $state(null);
	let date: string = $state('');
	let begin_time: string = $state('');
	let end_time: string = $state('');

	async function submitForm(event: Event) {
		event.preventDefault();

		if (absent_teacher_id === null) {
			// TODO: Show proper error message in the form
			alert('Per favore seleziona un professore.');
			return;
		}

        const body = {
            absent_teacher_id: absent_teacher_id,
            date,
            begin_time,
            end_time
        };

		const res = await client.POST('/absence', {
			body
		});

		if (res.response.ok) {
            await invalidateAll();

			// Successfully added absence
			open = false;

			// Reset form fields
			absent_teacher_id = null;
			date = '';
			begin_time = '';
			end_time = '';
		} else {
			// Todo: Show proper error message in the form
			alert("Errore durante l'aggiunta dell'assenza. Per favore riprova.");
		}
	}
</script>

<Drawer.Root
	onOpenChange={(o) => {
		if (o) hideTrigger = o;
	}}
	onOpenChangeComplete={(o) => {
		if (!o) hideTrigger = o;
	}}
	bind:open
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
	<Drawer.Content class="pb-3">
		<Drawer.Header>
			<Drawer.Title class="text-center text-3xl font-semibold">
				Aggiungi assenza professore
			</Drawer.Title>
		</Drawer.Header>

		<form onsubmit={submitForm} class="flex w-full flex-col items-center gap-4">
			<TeacherPicker bind:value={absent_teacher_id} />

            <!-- TODO: Get date from the page -->
			<Input type="date" placeholder="Del giorno" class="max-w-lg" id="date" bind:value={date} />

			<Input
				type="time"
				placeholder="Dalle ore"
				class="max-w-lg"
				id="begin_time"
				bind:value={begin_time}
                required
			/>

			<Input
				type="time"
				placeholder="Alle ore"
				class="max-w-lg"
				id="end_time"
				bind:value={end_time}
                required
			/>

			<Drawer.Footer class="flex w-full flex-row justify-center">
				<Button class="w-full max-w-lg" type="submit">Add</Button>
			</Drawer.Footer>
		</form>
	</Drawer.Content>
</Drawer.Root>

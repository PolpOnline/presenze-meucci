<script>
	import { Button } from '$components/ui/button/index.js';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer/index.js';
	import { Input } from '$components/ui/input/index.js';
	import PageSelector from '$components/PageSelector.svelte';
    import LucidePlus from '~icons/lucide/plus';
</script>

<main class="flex justify-center">
	<div class="w-full max-w-[600px]">
		<PageSelector class="my-4" />

		<Drawer.Trigger>
			<Button variant="secondary" size="icon" class="rounded-2 fixed right-5 bottom-5 size-12">
                <LucidePlus class="size-6" />
			</Button>
		</Drawer.Trigger>

		<Drawer.Content class="rounded-t-2xl border-t bg-background p-6 shadow-xl">
			<Drawer.Header class="">
				<Drawer.Title class="text-lg font-semibold">Aggiungi assenza professore</Drawer.Title>
			</Drawer.Header>
			<Input type="email" placeholder="email" class="max-w-xs" />

			<Drawer.Footer class="flex justify-end gap-2">
				<Button>Add</Button>
			</Drawer.Footer>
		</Drawer.Content>
	</div>
</main>

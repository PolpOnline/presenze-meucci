<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import LucideChevronLeft from '~icons/lucide/chevron-left';
	import LucideChevronRight from '~icons/lucide/chevron-right';
	import LucideCalendarSync from '~icons/lucide/calendar-sync';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils';
	import { type ClassValue } from 'clsx';
	import { toast } from 'svelte-sonner';
	import { DateTime } from 'luxon';
	import { capitalizeFirstLetter } from '$lib/utils/text';

	const {
		class: className,
		date,
		formattedDate
	}: { class?: ClassValue; date: string | null; formattedDate: string } = $props();

	const currentDate = $derived((date ? DateTime.fromISO(date) : DateTime.now()).startOf('day'));

	const prevTarget = $derived(currentDate.minus({ days: 1 }));
	const prevDateHref = $derived(
		prevTarget.equals(DateTime.now().startOf('day')) ? '?' : `?date=${prevTarget.toISODate()}`
	);
	const nextTarget = $derived(currentDate.plus({ days: 1 }));
	const nextDateHref = $derived(
		nextTarget.equals(DateTime.now().startOf('day')) ? '?' : `?date=${nextTarget.toISODate()}`
	);

	onMount(() => {
		document.addEventListener('keydown', keyHandler);
		return () => {
			document.removeEventListener('keydown', keyHandler);
		};
	});

	function keyHandler(event: KeyboardEvent) {
		if (event.key === 'ArrowRight') {
			goto(prevDateHref);
		}
		if (event.key === 'ArrowLeft') {
			goto(nextDateHref);
		}
		if (event.key === 'Escape') {
			toast.info('Reset to live view');
			goto('?');
		}
	}
</script>

<div class={cn(className, 'mb-3 flex items-center justify-between')}>
	<Button
		variant="outline"
		size="icon"
		href={prevDateHref}
		data-sveltekit-preload-data="hover"
		data-sveltekit-preload-code="eager"
		data-sveltekit-replacestate
		aria-label="Go to the previous day"
	>
		<LucideChevronLeft />
	</Button>

	<div class="text-lg font-medium tracking-wide">
		{capitalizeFirstLetter(formattedDate)}
	</div>

	<div>
		<Button variant="outline" size="icon" disabled={!date} aria-label="Reset to today">
			<LucideCalendarSync />
		</Button>
		<Button
			variant="outline"
			size="icon"
			href={nextDateHref}
			data-sveltekit-preload-data="hover"
			data-sveltekit-preload-code="eager"
			data-sveltekit-replacestate
			aria-label="Go to the next day"
		>
			<LucideChevronRight />
		</Button>
	</div>
</div>

<script lang="ts">
	import { type DateValue, DateFormatter, getLocalTimeZone } from '@internationalized/date';
	import { cn } from '$lib/utils.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Calendar } from '$lib/components/ui/calendar/index.js';
	// noinspection ES6UnusedImports
	import * as Popover from '$lib/components/ui/popover/index.js';
	import LucideCalendarIcon from '~icons/lucide/calendar-icon';

	const df = new DateFormatter('en-US', {
		dateStyle: 'long'
	});

	export let value: DateValue | undefined = undefined;
</script>

<Popover.Root>
	<Popover.Trigger>
		<Button
			variant="outline"
			class={cn('w-[280px] justify-start text-left font-normal', !value && 'text-muted-foreground')}
		>
			<LucideCalendarIcon class="mr-2 size-4" />
			{value ? df.format(value.toDate(getLocalTimeZone())) : 'Seleziona una data'}
		</Button>
	</Popover.Trigger>

	<Popover.Content class="w-auto p-0">
		<Calendar bind:value type="single" initialFocus />
	</Popover.Content>
</Popover.Root>

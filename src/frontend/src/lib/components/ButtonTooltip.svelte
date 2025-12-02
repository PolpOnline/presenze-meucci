<script lang="ts">
	import { Button } from '$components/ui/button/index';
	// noinspection ES6UnusedImports
	import * as Tooltip from '$components/ui/tooltip/index';
	import type { Snippet } from 'svelte';
	import type { ButtonProps } from '$components/ui/button';

	const { label, children, ...props }: { label: string; children?: Snippet } & ButtonProps =
		$props();
</script>

<Tooltip.Root>
	<Tooltip.Trigger>
		<Button aria-label={label} {...props}>
			{@render children?.()}
		</Button>
	</Tooltip.Trigger>
	<Tooltip.Content side="top" align="center">
		{label}
	</Tooltip.Content>
</Tooltip.Root>

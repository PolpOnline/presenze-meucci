<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { page } from '$app/state';
	import LucideChevronLeft from '~icons/lucide/chevron-left';
	import LucideChevronRight from '~icons/lucide/chevron-right';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils';
	import { type ClassValue } from 'clsx';
	import { toast } from 'svelte-sonner';

	const currentPage = $derived(Number(page.url.searchParams.get('page')) || 0);

	const prevPage = $derived(currentPage - 1 === 0 ? null : currentPage - 1);
	const prevPageHref = $derived(prevPage === null ? `?` : `?page=${prevPage}`);

	const nextPage = $derived(currentPage + 1 === 0 ? null : currentPage + 1);
	const nextPageHref = $derived(nextPage === null ? `?` : `?page=${nextPage}`);

	onMount(() => {
		document.addEventListener('keydown', keyHandler);
		return () => {
			document.removeEventListener('keydown', keyHandler);
		};
	});

	function keyHandler(event: KeyboardEvent) {
		if (event.key === 'ArrowRight') {
			goto(prevPageHref);
		}
		if (event.key === 'ArrowLeft') {
			goto(nextPageHref);
		}
		if (event.key === 'Escape') {
			toast.info('Reset to live view');
			goto('?');
		}
	}

	const { class: className }: { class?: ClassValue } = $props();
</script>

<div class={cn(className, 'mb-3 flex items-center justify-between')}>
	<Button
		variant="outline"
		size="icon"
		href={prevPageHref}
		data-sveltekit-preload-data="hover"
		data-sveltekit-preload-code="eager"
		data-sveltekit-replacestate
		aria-label="Go back in time"
	>
		<LucideChevronLeft />
	</Button>

	<!-- TODO: Add the current date -->

	<Button
		variant="outline"
		size="icon"
		href={nextPageHref}
		data-sveltekit-preload-data="hover"
		data-sveltekit-preload-code="eager"
		data-sveltekit-replacestate
		aria-label="Go forward in time"
	>
		<LucideChevronRight />
	</Button>
</div>

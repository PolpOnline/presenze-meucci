<script lang="ts">
	import '../app.css';
	import favicon from '$lib/images/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import Navbar from '$lib/components/Navbar.svelte';
	import { ProgressBar } from '@prgm/sveltekit-progress-bar';
	import { title } from '$lib/stores/title.store';
	// noinspection ES6UnusedImports
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import { PUBLIC_API_URL } from '$lib/api/public-api';

	let { children, data } = $props();
</script>

<svelte:head>
	<link rel="preconnect" href={PUBLIC_API_URL} />
	<link rel="icon" type="image/svg+xml" href={favicon} />
	<title>{$title}</title>
</svelte:head>

<ModeWatcher defaultMode="dark" />

<div data-vaul-drawer-wrapper>
	<ProgressBar class="text-white" zIndex={100} />

	<Tooltip.Provider>
		<Navbar loggedIn={data.loggedIn} />

		{@render children?.()}
	</Tooltip.Provider>
</div>

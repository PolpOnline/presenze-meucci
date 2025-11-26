<script lang="ts">
	import '../app.css';
	import favicon from '$lib/images/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer/index.js';
	import Navbar from '$lib/components/Navbar.svelte';
	import type { LoginStatus } from '../app';
	import { ProgressBar } from '@prgm/sveltekit-progress-bar';

	let { children } = $props();
	let loggedInEmail: string = $state('');
	let loginStatus: LoginStatus = $state('logged_out');
</script>

<svelte:head>
	<link rel="icon" type="image/svg+xml" href={favicon} />
</svelte:head>

<ModeWatcher defaultMode="dark" />

<div data-vaul-drawer-wrapper>
	<ProgressBar class="text-white" zIndex={100} />

	<Navbar {loggedInEmail} {loginStatus} />

	<Drawer.Root>
		{@render children?.()}
	</Drawer.Root>
</div>

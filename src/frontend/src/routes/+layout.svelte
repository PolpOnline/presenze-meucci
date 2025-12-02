<script lang="ts">
	import '../app.css';
	import favicon from '$lib/images/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import Navbar from '$lib/components/Navbar.svelte';
	import type { LoginStatus } from '../app';
	import { ProgressBar } from '@prgm/sveltekit-progress-bar';
	import { title } from '$lib/stores/title.store';

	let { children } = $props();
	let loggedInEmail: string = $state('');
	let loginStatus: LoginStatus = $state('logged_out');
</script>

<svelte:head>
	<link rel="icon" type="image/svg+xml" href={favicon} />
	<title>{$title}</title>
</svelte:head>

<ModeWatcher defaultMode="dark" />

<div data-vaul-drawer-wrapper>
	<ProgressBar class="text-white" zIndex={100} />

	<Navbar {loggedInEmail} {loginStatus} />

	{@render children?.()}
</div>

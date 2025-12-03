<script lang="ts">
	import LucideLogOut from '~icons/lucide/log-out';
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideSettings from '~icons/lucide/settings';
	import LucideGithub from '~icons/lucide/github';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import DropdownMenuLinkItem from '$lib/components/DropdownMenuLinkItem.svelte';
	import ITISMeucciLogo from '$lib/images/ITISMeucciLogo.svelte';
	import { client } from '$lib/api/api';
	import { goto, invalidate } from '$app/navigation';

	const { loggedIn }: { loggedIn: boolean } = $props();

	async function handleLogout() {
		try {
			const res = await client.GET('/logout');

			await invalidate('app:loggedIn');

			if (res.response.ok) {
				await goto('/auth/login');
			} else {
				console.error('Logout failed');
			}
		} catch (error) {
			console.error('An error occurred during logout:', error);
		}
	}
</script>

<nav class="grid h-20 grid-cols-12">
	<div class="grid-span-2 col-span-2 flex items-center">
		<a href="/">
			<ITISMeucciLogo class="ml-3 size-20 max-w-full" />
		</a>
	</div>
	<div class="col-span-8 flex items-center justify-center font-black">
		<a href="/" class="text-center text-3xl"> Presenze Meucci </a>
	</div>
	<div class="col-span-2 mr-3 flex items-center gap-1 justify-self-end">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger class={buttonVariants({ size: 'icon' })} aria-label="Navbar Menu">
				<LucideSettings />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content>
				{#if loggedIn}
					<DropdownMenu.Group>
						<DropdownMenu.Item class="text-destructive" onclick={handleLogout}>
							<LucideLogOut class="mr-2 size-4 text-destructive" />
							Logout
						</DropdownMenu.Item>
					</DropdownMenu.Group>
					<DropdownMenu.Separator />
				{/if}
				<DropdownMenuLinkItem href="https://github.com/PolpOnline/presenze-meucci" target="_blank">
					<LucideGithub class="mr-2 size-4" />
					GitHub
				</DropdownMenuLinkItem>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</nav>

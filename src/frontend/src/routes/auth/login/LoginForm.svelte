<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import PasswordInput from '$components/password_input/PasswordInput.svelte';
	import { Button } from '$components/ui/button';
	import { client } from '$lib/api/api';

	let isSubmitting = $state(false);
	let message = $state<string | null>(null);

	async function login(event: Event) {
		event.preventDefault();
		isSubmitting = true;
		message = null;

		const formData = new FormData(event.target as HTMLFormElement);

		try {
			const res = await client.POST('/login', {
				// @ts-expect-error We know the type is correct
				body: Object.fromEntries(formData)
			});

			if (res.error) {
				message = res.data || 'Login failed. Please try again.';
			} else {
				window.location.href = '/';
			}
		} catch (error) {
			message = 'An error occurred. Please try again. Details: ' + error;
		} finally {
			isSubmitting = false;
		}
	}
</script>

<form class="flex w-full flex-col" onsubmit={login}>
	<label class="mt-4" for="username"> Username </label>
	<Input class="my-2" name="username" autocomplete="username" required />

	<label class="my-3 flex items-center justify-between" for="password">
		Password
		<a href="" class="underline">Password Dimenticata</a>
	</label>
	<PasswordInput name="password" autocomplete="current-password" required />

	{#if message}
		<div class="text-destructive">{message}</div>
	{/if}
	<Button class="mt-8 w-full" disabled={isSubmitting} type="submit">
		{#if !isSubmitting}
			Login
		{:else}
			<LineMdLoadingLoop class="size-6" />
		{/if}
	</Button>
</form>

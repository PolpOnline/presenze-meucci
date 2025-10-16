<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { formSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zod4Client } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import PasswordInput from '$components/password_input/PasswordInput.svelte';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';


	const {
		data
	}: {
		data: SuperValidated<Infer<FormSchema>>;
	} = $props();

	const form = superForm(data, {
		validators: zod4Client(formSchema),
		onUpdated: ({ form: f }) => {
			if (f.valid) {
				toast.success("Login eseguito con successo!");

				goto('/');
			}
		}
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form method="POST" use:enhance>
	<Form.Field {form} name="username">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>
					Username
				</Form.Label>
				<Input {...props} bind:value={$formData.username} autocomplete="username" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>
					Password
				</Form.Label>
				<PasswordInput {...props} bind:value={$formData.password} autocomplete="current-password" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-destructive">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			Login
		{:else}
			<LineMdLoadingLoop class="size-6" />
		{/if}
	</Form.Button>
</form>

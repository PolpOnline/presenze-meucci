import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api.server';
import { redirect } from '@sveltejs/kit';
import { StatusCodes } from 'http-status-codes';

export const load: PageServerLoad = async (event) => {
	await client.GET('/logout', {
		fetch: event.fetch
	});

	redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
};
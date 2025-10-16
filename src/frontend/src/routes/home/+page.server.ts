import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api.server';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data, response } = await client.GET('/teachers/can_be_absent', { fetch });
	console.log(await client.GET('/teachers/can_be_absent', { fetch }));

	if (response.status === StatusCodes.FORBIDDEN) {
		error(StatusCodes.FORBIDDEN, getReasonPhrase(StatusCodes.FORBIDDEN));
	}

	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	return {
		professors: data
	};
};
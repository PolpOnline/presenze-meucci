import type { PageLoad } from './$types';
import { client } from '$lib/api/api';

export const load: PageLoad = async (event) => {
	const absences = await client.GET('/absence', {
		fetch: event.fetch
	});

	return {
		absences: absences.data
	};
};

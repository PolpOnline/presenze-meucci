import type { PageLoad } from './$types';
import { client } from '$lib/api/api';

export const load: PageLoad = async (event) => {
	const params = new URLSearchParams(event.url.search);

	const date = params.get('date');

	const absences = await client.GET('/absence', {
		fetch: event.fetch,
		params: {
			query: {
				date: date ?? undefined
			}
		}
	});

	return {
		date: date,
		absences: absences.data
	};
};

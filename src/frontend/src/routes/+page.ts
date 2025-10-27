import type { PageLoad } from './$types';
import { client } from '$lib/api/api';

export const load: PageLoad = async (event) => {
    await client.GET('/logout', {
        fetch: event.fetch
    });

};
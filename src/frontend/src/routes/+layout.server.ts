import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies, depends }) => {
	depends('app:loggedIn');

	return {
		loggedIn: !!cookies.get('meucci_presenze_id')
	};
};

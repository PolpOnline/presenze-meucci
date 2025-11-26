export const ssr = false;
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = ({ url }) => {
	return {
		pathname: url.pathname
	};
};

export const ssr = false;
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = ({ url, data }) => {
	return {
		pathname: url.pathname,
		loggedIn: data.loggedIn
	};
};

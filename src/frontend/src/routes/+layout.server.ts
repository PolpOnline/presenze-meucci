// src/routes/+layout.server.ts
import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ cookies, url }) => {

    const sessionToken = cookies.get('session');

    const loggedIn = !!sessionToken;

    if (!loggedIn && url.pathname !== '/login') {
        throw redirect(302, '/login');
    }

    if (loggedIn && url.pathname === '/login') {
        throw redirect(302, '/home');
    }

    return { loggedIn };
};


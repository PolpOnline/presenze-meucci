import { dev } from '$app/environment';

export const PUBLIC_API_URL = dev ? 'http://localhost:3000' : 'https://example-api.com';

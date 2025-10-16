import createClient from 'openapi-fetch';
import type { paths } from '$lib/api/schema';
import { PUBLIC_API_URL } from './public-api';

export const client = createClient<paths>({ baseUrl: PUBLIC_API_URL });

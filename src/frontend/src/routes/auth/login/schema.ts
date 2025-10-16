import { z } from 'zod';

export const formSchema = z.object({
	username: z.string().trim().toLowerCase(),
	password: z.string()
});

export type FormSchema = typeof formSchema;

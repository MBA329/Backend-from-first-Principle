import { z } from 'zod';

// optional + then 7-15 digits (country code + national no.)
const phoneRe = /^\+?[0-9]{7,15}$/;

export const ContactSchema = z.object({
  email: z.string().email(), // local @ domain.tld
  phone: z.string().regex(phoneRe, "invalid phone number format"),
  date: z.string().regex(/^\d{4}-\d{2}-\d{2}$/, "only accepts YYYY-MM-DD") // YYYY-MM-DD
});
export type Contact = z.infer<typeof ContactSchema>;

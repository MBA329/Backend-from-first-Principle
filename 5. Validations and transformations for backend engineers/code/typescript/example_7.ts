import { z } from 'zod';

export const ContactSchema = z.object({
  email: z.string().email().transform(v => v.trim().toLowerCase()),
  phone: z.string().transform(v => {
    v = v.trim();
    return v.startsWith('+') ? v : '+' + v;
  })
});
export type Contact = z.infer<typeof ContactSchema>;

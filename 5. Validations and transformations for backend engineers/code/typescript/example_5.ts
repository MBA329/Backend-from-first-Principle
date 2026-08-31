import { z } from 'zod';

export const SignupSchema = z.object({
  password: z.string().min(8),
  passwordConfirmation: z.string(),
  married: z.boolean(),
  partner: z.string().optional()
}).refine((data) => data.password === data.passwordConfirmation, {
  message: "passwords don't match",
  path: ["passwordConfirmation"]
}).refine((data) => {
  if (data.married && !data.partner) return false;
  return true;
}, {
  message: "partner name is required when married is true",
  path: ["partner"]
});
export type Signup = z.infer<typeof SignupSchema>;

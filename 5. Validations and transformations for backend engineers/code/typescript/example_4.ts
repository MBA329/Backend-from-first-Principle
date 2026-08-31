import { z } from 'zod';

export const ProfileSchema = z.object({
  dateOfBirth: z.string().refine((val) => {
    const dob = new Date(val);
    if (isNaN(dob.getTime())) return false;
    if (dob > new Date()) return false;
    return true;
  }, { message: "date of birth cannot be in the future" }),
  age: z.number().min(1).max(120) // gte/lte cover the "430 is impossible" semantic bound
});
export type Profile = z.infer<typeof ProfileSchema>;

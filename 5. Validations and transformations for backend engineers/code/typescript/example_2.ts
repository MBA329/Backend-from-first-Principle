import { z } from 'zod';

export const TypePayloadSchema = z.object({
  stringField: z.string(),
  numberField: z.number(),
  // recursive: EVERY element a string
  arrayField: z.array(z.string()),
  boolField: z.boolean(),
});
export type TypePayload = z.infer<typeof TypePayloadSchema>;

// Strict, explicit type errors are caught by Zod during parsing.
// Like pydantic, Zod catches type mismatches.

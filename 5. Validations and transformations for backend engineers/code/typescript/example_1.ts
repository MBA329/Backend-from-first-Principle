import { z } from 'zod';
import { Request, Response } from 'express';

// CreateBook is the SCHEMA. Each annotation = one rule
const CreateBookSchema = z.object({
  name: z.string().min(5).max(100)
});
type CreateBook = z.infer<typeof CreateBookSchema>;

// runPipeline decodes then validates; returns 400-ready messages.
export function runPipeline(req: Request, res: Response): CreateBook | null {
  try {
    // constructing the model runs all three layers
    // in order: existence -> type -> constraint
    return CreateBookSchema.parse(req.body);
  } catch (e) {
    if (e instanceof z.ZodError) {
      const messages = e.errors.map(err => `${err.path.join('.')}: ${err.message}`);
      res.status(400).json({ detail: messages });
      return null;
    }
    throw e;
  }
}

import express, { Request, Response } from 'express';
import { z } from 'zod';

const app = express();
app.use(express.json());

// ---- schema (the gate) ----
const CreateBookSchema = z.object({
  name: z.string().min(5).max(100).transform(v => v.trim())
});
type CreateBook = z.infer<typeof CreateBookSchema>;

interface Book {
  id: number;
  name: string;
}

// ---- service + repository (sketched) ----
async function createBook(name: string): Promise<Book> {
  // service logic ... repository INSERT ... then:
  return { id: 1, name };
}

// ---- CONTROLLER ----
app.post("/api/books", async (req: Request, res: Response) => {
  try {
    // === GATE ,  runs before ANY business logic ===
    const body = CreateBookSchema.parse(req.body);

    // === only now: business logic (service -> repo) ===
    const book = await createBook(body.name);
    res.status(201).json(book);
  } catch (error) {
    if (error instanceof z.ZodError) {
      res.status(400).json({ error: error.errors });
    } else {
      res.status(500).json({ error: "could not create book" });
    }
  }
});

app.listen(8080);

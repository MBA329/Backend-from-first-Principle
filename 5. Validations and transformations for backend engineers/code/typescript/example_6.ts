import { z } from 'zod';

export const PaginationSchema = z.object({
  // Zod coercing the incoming string "2" into int 2 (transform), 
  // THEN enforces gt/lt (validate)
  page: z.coerce.number().int().min(1).max(499),
  limit: z.coerce.number().int().min(1).max(9999)
});
export type Pagination = z.infer<typeof PaginationSchema>;

export function parsePagination(q: any): Pagination {
    return PaginationSchema.parse(q);
}

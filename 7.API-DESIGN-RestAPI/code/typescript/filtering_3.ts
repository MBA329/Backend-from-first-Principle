import { Request, Response } from 'express';

// GET /v1/organizations?status=active&sortBy=name&sortOrder=ascending&page=1&limit=10
export async function listOrganizations(req: Request, res: Response) {
    // --- sane defaults: never crash if the client omits params ---
    const page = parseInt(req.query.page as string) || 1;
    const limit = parseInt(req.query.limit as string) || 10;
    const sortBy = (req.query.sortBy as string) || 'createdAt';
    const sortOrder = (req.query.sortOrder as string) || 'descending';

    const filters: Record<string, string> = {};
    if (req.query.status) {
        filters['status'] = req.query.status as string; // ?status=active
    }

    const { rows, total } = await store.query(filters, sortBy, sortOrder, page, limit);
    const totalPages = Math.ceil(total / limit); // ceil division

    return res.status(200).json({
        data: rows,
        total: total,
        page: page,
        totalPages: totalPages
    });
}

// Mock store for compilation
const store = {
    query: async (filters: any, sortBy: string, sortOrder: string, page: number, limit: number): Promise<any> => ({ rows: [], total: 0 })
};

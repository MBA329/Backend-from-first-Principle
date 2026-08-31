import { Request, Response } from 'express';

export async function updateOrganization(req: Request, res: Response) {
    const { id } = req.params;
    const org = await store.get(id);
    
    if (!org) {
        return res.status(404).json({ error: { message: "organization not found" } });
    }
    
    // optimistic concurrency: reject stale writes
    const match = req.get('If-Match');
    if (match && match !== org.ETag) {
        return res.status(412).json({ error: { message: "resource changed; re-fetch and retry" } });
    }
    
    const patch = req.body; // only the fields to change
    const updated = await store.patch(id, patch); // merge, don't replace
    
    res.setHeader('ETag', updated.ETag);
    return res.status(200).json(updated); // 200 + updated entity
}

// Mock store for compilation
const store = {
    get: async (id: string): Promise<any> => ({ ETag: "123" }),
    patch: async (id: string, data: any): Promise<any> => ({ ETag: "124" })
};

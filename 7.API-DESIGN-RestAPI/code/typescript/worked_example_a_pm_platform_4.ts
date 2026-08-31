import { Router, Request, Response } from 'express';

export function registerOrgRoutes(router: Router) {
    // list + create share the collection URL, split by method
    router.get('/v1/organizations', listOrganizations);
    router.post('/v1/organizations', createOrganization);

    // get-one / update / delete share /:id, split by method
    router.get('/v1/organizations/:id', getOrganization);
    router.patch('/v1/organizations/:id', updateOrganization);
    router.delete('/v1/organizations/:id', deleteOrganization);

    // custom action: verb at the end of a specific resource
    router.post('/v1/organizations/:id/archive', archiveOrganization);
}

async function createOrganization(req: Request, res: Response) {
    const { name, description } = req.body;
    let { status } = req.body;

    if (!status) {
        status = "active"; // sane default, don't force the client to send the obvious
    }
    const org = await store.insert({ name, status, description }); // id, createdAt set server-side
    return res.status(201).json(org); // 201 Created + the new entity
}

async function deleteOrganization(req: Request, res: Response) {
    await store.delete(req.params.id);
    return res.status(204).send(); // No Content
}

async function archiveOrganization(req: Request, res: Response) {
    const org = await store.archive(req.params.id); // flips status + cascades: projects, tasks, emails...
    return res.status(200).json(org); // custom action -> 200, NOT 201
}

// Mocks for compilation
const listOrganizations = (req: Request, res: Response) => {};
const getOrganization = (req: Request, res: Response) => {};
const updateOrganization = (req: Request, res: Response) => {};
const store = {
    insert: async (data: any): Promise<any> => ({ id: "org_1", ...data }),
    delete: async (id: string): Promise<void> => {},
    archive: async (id: string): Promise<any> => ({ id, status: "archived" })
};

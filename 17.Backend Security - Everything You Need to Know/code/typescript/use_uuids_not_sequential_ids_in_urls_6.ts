import { Request, Response, NextFunction } from "express";

interface User {
    role: string;
}

export function requireRole(role: string) {
    return (req: Request, res: Response, next: NextFunction) => {
        const user = req.user as User; // Assuming user is attached to req by auth middleware
        if (!user || user.role !== role) {
            res.status(403).send("forbidden");
            return;
        }
        next();
    };
}

// Router setup
// app.get("/admin/invoices", requireAuth, requireRole("admin"), adminInvoicesHandler);

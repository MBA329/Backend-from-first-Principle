import { Request, Response, NextFunction } from 'express';

interface User {
    id: string;
    role: string;
}

// RequireRole runs AFTER an auth middleware that set "user".
export function requireRole(...roles: string[]) {
    const allowed = new Set(roles);
    
    return function(req: Request, res: Response, next: NextFunction): void {
        const u = (req as any).user as User;
        
        if (!u || !allowed.has(u.role)) {
            res.status(403).send('forbidden'); // 403
            return;
        }
        
        next();
    };
}

// usage: only admins reach the dead-zone handler
// app.use('/admin/deadzone', requireJWT, requireRole('admin'), deadZoneHandler);

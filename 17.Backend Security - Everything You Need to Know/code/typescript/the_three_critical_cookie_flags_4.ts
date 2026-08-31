import { Response } from "express";

export function setSessionCookie(res: Response, sessionID: string) {
    res.cookie("session_id", sessionID, {
        httpOnly: true,          // JS cannot access
        secure: true,            // HTTPS only
        sameSite: "strict",      // no cross-site
        maxAge: 7 * 24 * 3600 * 1000, // 7 days in ms
        path: "/",
    });
}

import { Request, Response } from "express";
import crypto from "crypto";
import { Pool } from "pg";
import { Redis } from "ioredis";

// verifyArgon2 mock
const verifyArgon2 = async (password: string, hash: string) => true;

export function loginHandler(db: Pool, redis: Redis) {
    return async (req: Request, res: Response) => {
        const { email, password } = req.body;

        // 1. Validate format (first line of defence)
        if (!email || !email.includes("@") || !password || password.length < 8) {
            res.status(400).send("invalid credentials");
            return;
        }

        try {
            // 2. Parameterised query, no SQL injection possible
            const result = await db.query(
                "SELECT id, password_hash FROM users WHERE email = $1",
                [email]
            );

            if (result.rows.length === 0) {
                res.status(401).send("invalid email or password");
                return;
            }

            const { id: userID, password_hash: hashedPass } = result.rows[0];

            // 3. Generic error, never reveal whether email exists
            if (!(await verifyArgon2(password, hashedPass))) {
                res.status(401).send("invalid email or password");
                return;
            }

            // 4. Cryptographically secure session ID
            const sessionID = crypto.randomBytes(32).toString("base64url");

            // 5. Store session in Redis with metadata
            await redis.set(`session:${sessionID}`, userID, "EX", 7 * 24 * 3600);

            // 6. Secure cookie, HttpOnly, Secure, SameSite=Strict
            res.cookie("session_id", sessionID, {
                httpOnly: true,
                secure: true,
                sameSite: "strict",
                maxAge: 7 * 24 * 3600 * 1000, // ms
            });
            res.status(200).send("OK");
        } catch (err) {
            res.status(500).send("server error");
        }
    };
}

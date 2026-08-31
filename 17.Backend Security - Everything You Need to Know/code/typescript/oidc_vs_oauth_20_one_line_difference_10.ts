import { Request, Response } from "express";
import { Issuer, generators } from "openid-client";

// Setup
export async function setupClient() {
  const googleIssuer = await Issuer.discover("https://accounts.google.com");
  return new googleIssuer.Client({
    client_id: process.env.GOOGLE_CLIENT_ID!,
    client_secret: process.env.GOOGLE_CLIENT_SECRET!,
    redirect_uris: ["https://yourapp.com/auth/callback"],
    response_types: ["code"],
  });
}

// Mocks
const getSessionState = (req: Request) => req.cookies.state;
const upsertUser = async (sub: string, email: string) => "user123";
const setSecureSessionCookie = (res: Response, userID: string) => res.cookie("session_id", "val");

export async function callbackHandler(client: any) {
  return async (req: Request, res: Response) => {
    // 1. Verify state matches what we stored in session (CSRF protection)
    const params = client.callbackParams(req);
    if (params.state !== getSessionState(req)) {
      res.status(400).send("invalid state");
      return;
    }

    try {
      // 2. Exchange code for tokens (server-to-server)
      // 3. Verify ID token signature + aud + exp happens automatically in callback()
      const tokenSet = await client.callback(
        "https://yourapp.com/auth/callback",
        params,
        { state: getSessionState(req) }
      );

      // 4. Extract claims
      const claims = tokenSet.claims();

      // 5. Upsert user in DB, create session, set cookie
      const userID = await upsertUser(claims.sub, claims.email as string);
      setSecureSessionCookie(res, userID);
      res.redirect("/dashboard");
    } catch (err) {
      res.status(401).send("invalid token");
    }
  };
}

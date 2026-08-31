import crypto from 'crypto';
import fetch from 'node-fetch';

interface TokenResp {
    access_token: string;
    token_type: string;
    expires_in: number;
    refresh_token?: string;
}

// Build the PKCE pair before redirecting the user.
export function newPKCE(): { verifier: string; challenge: string } {
    const b = crypto.randomBytes(32);
    const verifier = b.toString('base64url');
    
    const sum = crypto.createHash('sha256').update(verifier).digest();
    const challenge = sum.toString('base64url');
    
    return { verifier, challenge };
}

// Leg 2: exchange the code (send code_verifier, not the secret).
export async function exchange(code: string, verifier: string): Promise<TokenResp> {
    const form = new URLSearchParams({
        grant_type: 'authorization_code',
        code: code,
        redirect_uri: 'https://notes.app/callback',
        client_id: 'note_app',
        code_verifier: verifier
    });
    
    const resp = await fetch('https://auth.example/token', {
        method: 'POST',
        body: form,
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        }
    });
    
    if (!resp.ok) {
        throw new Error(`HTTP error! status: ${resp.status}`);
    }
    
    return await resp.json() as TokenResp;
}

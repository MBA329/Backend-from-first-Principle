import jwt from 'jsonwebtoken';

const secret = 'keep-this-very-secret';

// mint a self-contained token carrying the claims
export function sign(userID: string, role: string): string {
    const claims = {
        sub: userID,                               // user id
        role: role,                                // for authorization
        iat: Math.floor(Date.now() / 1000),        // issued at
        exp: Math.floor(Date.now() / 1000) + 3600  // expiry (1 hour)
    };
    return jwt.sign(claims, secret, { algorithm: 'HS256' });
}

// verify signature + expiry; returns the claims if valid
export function verify(tokenStr: string): any {
    try {
        // algorithms restricts to HS256 to stop "alg: none" attacks
        return jwt.verify(tokenStr, secret, { algorithms: ['HS256'] });
    } catch (err) {
        throw new Error('Invalid signature or expired token');
    }
}

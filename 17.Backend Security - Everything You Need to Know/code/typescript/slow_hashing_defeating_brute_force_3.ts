import * as crypto from "crypto";
import { promisify } from "util";

const randomBytes = promisify(crypto.randomBytes);

export async function hashPassword(password: string): Promise<string> {
  // Generate cryptographically random 16-byte salt
  const salt = await randomBytes(16);

  // Argon2id params equivalent for Node's scrypt or using a package.
  // We'll use scrypt for standard node built-ins, or we can use argon2 package.
  return new Promise((resolve, reject) => {
    crypto.scrypt(password, salt, 64, { N: 16384, r: 8, p: 1 }, (err, derivedKey) => {
      if (err) return reject(err);
      
      // Store: $salt$hash (both needed to verify)
      const encoded = `${salt.toString("base64")}$${derivedKey.toString("base64")}`;
      resolve(encoded);
    });
  });
}

export async function verifyPassword(password: string, stored: string): Promise<boolean> {
  // Re-hash with stored salt, compare, never compare raw hashes with ==
  const [saltB64, hashB64] = stored.split("$");
  const salt = Buffer.from(saltB64, "base64");
  const expectedHash = Buffer.from(hashB64, "base64");

  return new Promise((resolve, reject) => {
    crypto.scrypt(password, salt, 64, { N: 16384, r: 8, p: 1 }, (err, derivedKey) => {
      if (err) return reject(err);
      // Use timingSafeEqual to prevent timing attacks
      resolve(crypto.timingSafeEqual(expectedHash, derivedKey));
    });
  });
}

use hmac::{Hmac, Mac};
use sha2::Sha256;
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use subtle::ConstantTimeEq;
use argon2::{Argon2, Params, PasswordHasher};

lazy_static::lazy_static! {
    static ref PEPPER: Vec<u8> = std::env::var("PASSWORD_PEPPER").unwrap_or_default().into_bytes(); // secret, NOT in DB
}

// cost parameters ,  tune so one hash ~250ms
const A_TIME: u32 = 3;           // iterations
const A_MEMORY: u32 = 64 * 1024; // 64 MB, memory-hard
const A_THREADS: u32 = 4;
const A_KEY_LEN: usize = 32;

fn peppered(pw: &str) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(&PEPPER).expect("HMAC can take key of any size");
    mac.update(pw.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

// returns an encoded string: salt + params + hash
pub fn hash_argon(pw: &str) -> String {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt); // unique per password
    
    let params = Params::new(A_MEMORY, A_TIME, A_THREADS, Some(A_KEY_LEN)).unwrap();
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    
    let mut hash = vec![0u8; A_KEY_LEN];
    argon2.hash_password_into(&peppered(pw), &salt, &mut hash).unwrap();
    
    format!("argon2id${}${}${}${}${}", A_TIME, A_MEMORY, A_THREADS,
        STANDARD_NO_PAD.encode(&salt),
        STANDARD_NO_PAD.encode(&hash))
}

// re-derive with the SAME salt+params, then constant-time compare
pub fn verify_argon(encoded: &str, pw: &str) -> bool {
    let parts: Vec<&str> = encoded.split('$').collect();
    if parts.len() != 6 || parts[0] != "argon2id" {
        return false;
    }
    
    let t: u32 = parts[1].parse().unwrap();
    let mem: u32 = parts[2].parse().unwrap();
    let th: u32 = parts[3].parse().unwrap();
    let salt = STANDARD_NO_PAD.decode(parts[4]).unwrap();
    let want = STANDARD_NO_PAD.decode(parts[5]).unwrap();
    
    let params = Params::new(mem, t, th, Some(want.len())).unwrap();
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    
    let mut got = vec![0u8; want.len()];
    if argon2.hash_password_into(&peppered(pw), &salt, &mut got).is_err() {
        return false;
    }
    
    got.ct_eq(&want).into()
}

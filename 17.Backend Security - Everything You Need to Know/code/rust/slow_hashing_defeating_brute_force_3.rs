use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Generate cryptographically random 16-byte salt
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id params: default handles time, memory, threads
    let argon2 = Argon2::default();

    // Store: $argon2id$v=19$m=4096,t=3,p=1$salt$hash (both needed to verify)
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, stored: &str) -> bool {
    // Re-hash with stored salt, compare, never compare raw hashes with ==
    let parsed_hash = match PasswordHash::new(stored) {
        Ok(h) => h,
        Err(_) => return false,
    };
    
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const SECRET: &[u8] = b"keep-this-very-secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub role: String, // for authorization
    pub iat: u64, // issued at
    pub exp: u64, // expiry
}

// mint a self-contained token carrying the claims
pub fn sign(user_id: &str, role: &str) -> Result<String, Error> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        iat: now,
        exp: now + 3600,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

// verify signature + expiry; returns the claims if valid
pub fn verify(token_str: &str) -> Result<Claims, Error> {
    // pin the algorithm to stop "alg: none" attacks
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = true;
    
    let token_data = decode::<Claims>(
        token_str,
        &DecodingKey::from_secret(SECRET),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

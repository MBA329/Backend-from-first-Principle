use jsonwebtoken::{decode, Validation, DecodingKey, errors::Error};
use serde::{Deserialize, Serialize};
use redis::{AsyncCommands, Client};
use rand::RngCore;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Define claims as needed
    pub sub: String,
}

// Verify with a PUBLIC key, pinning RS256 (blocks alg:none & key confusion)
pub fn verify_rs256(token_str: &str, pub_key: &[u8]) -> Result<Claims, Error> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    let token_data = decode::<Claims>(
        token_str,
        &DecodingKey::from_rsa_pem(pub_key)?,
        &validation,
    )?;
    Ok(token_data.claims)
}

pub fn new_session_id() -> String {
    let mut b = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut b); // cryptographically random
    hex::encode(b)
}

// Rotate a refresh token; detect reuse of an already-spent one.
pub async fn refresh(rdb: &Client, presented: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut con = rdb.get_async_connection().await?;
    
    let family: Option<String> = con.get(format!("rt:{}", presented)).await?;
    
    let family = match family {
        Some(f) => f,
        None => {
            // not a live token ,  was it a previously-spent one? -> theft
            let spent: Option<String> = con.get(format!("spent:{}", presented)).await?;
            if let Some(fam) = spent {
                let _: () = con.del(format!("family:{}", fam)).await?; // revoke the whole family
                return Err("refresh reuse detected".into());
            }
            return Err("invalid refresh token".into());
        }
    };
    
    let _: () = con.del(format!("rt:{}", presented)).await?; // consume the live token
    let _: () = con.set_ex(format!("spent:{}", presented), &family, 14 * 24 * 60 * 60).await?; // remember it
    
    let new_refresh = new_session_id();
    let _: () = con.set_ex(format!("rt:{}", new_refresh), &family, 14 * 24 * 60 * 60).await?;
    
    Ok(new_refresh)
}

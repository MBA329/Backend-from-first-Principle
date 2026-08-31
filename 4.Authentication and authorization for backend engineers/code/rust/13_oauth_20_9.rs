use rand::RngCore;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResp {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
}

// Build the PKCE pair before redirecting the user.
pub fn new_pkce() -> (String, String) {
    let mut b = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut b);
    
    let verifier = URL_SAFE_NO_PAD.encode(&b);
    
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let sum = hasher.finalize();
    
    let challenge = URL_SAFE_NO_PAD.encode(&sum);
    
    (verifier, challenge)
}

// Leg 2: exchange the code (send code_verifier, not the secret).
pub async function exchange(code: &str, verifier: &str) -> Result<TokenResp, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", "https://notes.app/callback"),
        ("client_id", "note_app"),
        ("code_verifier", verifier),
    ];
    
    let resp = client.post("https://auth.example/token")
        .form(&params)
        .send()
        .await?;
        
    let token: TokenResp = resp.json().await?;
    
    Ok(token)
}

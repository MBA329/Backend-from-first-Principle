use actix_web::{HttpResponse, web};
use bcrypt::verify;
use std::time::{Instant, Duration};

pub struct User {
    pub id: String,
    pub hash: String,
}

// dummyHash: a fixed, precomputed bcrypt hash, used to equalize timing.
const DUMMY_HASH: &str = "$2a$12$abcdefghijklmnopqrstuv0000000000000000000000000000000000";

fn lookup_by_email(email: &str) -> Option<User> {
    // DB lookup mock
    None
}

pub async fn authenticate(email: String, pw: String) -> HttpResponse {
    let start = Instant::now();
    let floor = Duration::from_millis(250); // equalize every path

    // In Rust we can't easily use defer for async sleep, so we'll compute it at the end of every branch
    // Or we could wrap the logic in a block
    
    let result = (|| {
        let user = lookup_by_email(&email);
        if user.is_none() {
            // still run a dummy hash so step-3 cost is paid anyway
            let _ = verify(&pw, DUMMY_HASH);
            return HttpResponse::Unauthorized().body("authentication failed");
        }
        
        let u = user.unwrap();
        // bcrypt verify is constant-time internally
        if !verify(&pw, &u.hash).unwrap_or(false) {
            return HttpResponse::Unauthorized().body("authentication failed");
        }
        
        // success: issue session / JWT ...
        HttpResponse::Ok().body("ok")
    })();
    
    // pad so all outcomes take ~the same time
    let duration = start.elapsed();
    if duration < floor {
        tokio::time::sleep(floor - duration).await;
    }
    
    result
}

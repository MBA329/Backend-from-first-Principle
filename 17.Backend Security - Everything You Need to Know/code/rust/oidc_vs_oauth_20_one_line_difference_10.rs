use actix_web::{web, HttpResponse, Responder, HttpRequest};
use oauth2::{
    ClientId, ClientSecret, RedirectUrl, reqwest::async_http_client, AuthorizationCode, TokenResponse,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    IssuerUrl, Nonce,
};
use std::env;

// Setup
pub async fn setup_oidc() -> CoreClient {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://accounts.google.com".to_string()).unwrap(),
        async_http_client,
    )
    .await
    .unwrap();

    CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap())),
    )
    .set_redirect_uri(RedirectUrl::new("https://yourapp.com/auth/callback".to_string()).unwrap())
}

// Mocks
fn get_session_state(_req: &HttpRequest) -> String { String::new() }
fn upsert_user(_sub: &str, _email: &str) -> String { "user123".to_string() }
fn set_secure_session_cookie(_res: &mut HttpResponse, _user_id: &str) {}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}

pub async fn callback_handler(
    req: HttpRequest,
    query: web::Query<CallbackQuery>,
    client: web::Data<CoreClient>,
) -> impl Responder {
    // 1. Verify state matches what we stored in session (CSRF protection)
    if query.state != get_session_state(&req) {
        return HttpResponse::BadRequest().body("invalid state");
    }

    // 2. Exchange code for tokens (server-to-server)
    let token_response = match client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
    {
        Ok(t) => t,
        Err(_) => return HttpResponse::Unauthorized().body("invalid token"),
    };

    // 3. Verify ID token signature + aud + exp
    let id_token = match token_response.id_token() {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("invalid token"),
    };

    let claims = match id_token.claims(&client.id_token_verifier(), &Nonce::new(String::new())) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("invalid token"),
    };

    // 4. Extract claims
    let email = claims.email().map(|e| e.as_str()).unwrap_or("");
    let sub = claims.subject().as_str();

    // 5. Upsert user in DB, create session, set cookie
    let user_id = upsert_user(sub, email);
    
    let mut res = HttpResponse::Found()
        .append_header(("Location", "/dashboard"))
        .finish();
        
    set_secure_session_cookie(&mut res, &user_id);
    
    res
}

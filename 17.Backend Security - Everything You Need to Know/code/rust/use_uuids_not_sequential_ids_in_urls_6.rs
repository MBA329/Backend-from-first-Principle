use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest, HttpResponse, Error};
use std::future::{ready, Ready};

#[derive(Clone)]
pub struct User {
    pub role: String,
}

pub struct RequireRole(pub String);

impl FromRequest for RequireRole {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Typically, user is stored in req extensions by auth middleware
        if let Some(user) = req.extensions().get::<User>() {
            // Role checking happens in the handler or via a macro/guard,
            // but for simplicity we just return RequireRole and handler validates it,
            // or we can reject right here if the role doesn't match a static string.
            // Actix guards are better for this.
            ready(Ok(RequireRole(user.role.clone())))
        } else {
            ready(Err(actix_web::error::ErrorForbidden("forbidden")))
        }
    }
}

// Router setup example using Guards
// App::new()
//     .route("/admin/invoices", web::get()
//         .guard(fn_guard(|req| {
//             let ext = req.extensions();
//             if let Some(user) = ext.get::<User>() {
//                 user.role == "admin"
//             } else {
//                 false
//             }
//         }))
//         .to(admin_invoices_handler)
//     )

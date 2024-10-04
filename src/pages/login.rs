use actix_web::{ get, web, Error, HttpResponse, Responder, http };
use actix_identity::IdentityMiddleware;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String
}

/// Authenticates the user and creates a session.
pub async fn auth_user(form_data: web::Form<LoginForm>) -> HttpResponse {
    let form = form_data.into_inner();
    println!("username: {:?} || password: {}", form.username, form.password);
    HttpResponse::Ok().finish()
}

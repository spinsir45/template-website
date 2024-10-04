use core::hash;
use std::hash::{ DefaultHasher, Hash, Hasher };
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
    // Put form data into struct
    let form = form_data.into_inner();

    // Hash the password
    let mut hasher = DefaultHasher::new();
    form.password.hash(&mut hasher);
    let _value = hasher.finish();

    // Check if the user is an authentic user

    
    // Return response
    HttpResponse::Ok().finish()
}

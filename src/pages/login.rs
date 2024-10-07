use std::hash::{ DefaultHasher, Hash, Hasher };
use actix_web::{ web, HttpResponse };
use actix_session::Session;
use serde::{Deserialize, Serialize};
use toml::value::Value;

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String
}

/// Authenticates the user and creates a session.
pub async fn auth_user(form_data: web::Form<LoginForm>, session: Session) -> HttpResponse {
    let mut authorized: bool = false;
    // Put form data into struct
    let form = form_data.into_inner();

    // Hash the password
    let mut hasher = DefaultHasher::new();
    form.password.hash(&mut hasher);
    let hashed_password = hasher.finish();

    // Check if the user is an authentic user
    let toml_string = std::fs::read_to_string("/home/spinsir/Projects/template-website/Users.toml").unwrap();
    let toml_value: Value = toml::from_str(&toml_string).unwrap();
    let table = toml_value.as_table().unwrap();
    let users = table.get("users").unwrap();
    // Check if username if found in the toml file
    if let Some(value) = users.get(&form.username) {
        // Check if the password matches
        if value.as_str().unwrap() == hashed_password.to_string() {
            // Check if a session was able to be established
            if let Err(_) = session.insert("user", form.username) {
                return HttpResponse::Found().append_header(("Location", "/login")).body("Failed to establish a session")
            }
            authorized = true;
        }
    }
    
    // Return response
    if authorized {
        HttpResponse::Found().append_header(("Location", "/")).finish()
    } else {
        HttpResponse::Found().append_header(("Location", "/login")).body("Invalid Username or Password")
    }
}

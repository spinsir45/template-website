mod pages;

use actix_web::{web, App, HttpServer, cookie::Key};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_files as fs;
use tera::Tera;

use crate::pages::login::auth_user;
use crate::pages::dashboard::get_dashboard;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();
    let tera = Tera::new("./frontend/pages/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            .service(fs::Files::new("/login", "./frontend").index_file("pages/login.html"))
            .route("/auth", web::post().to(auth_user))
            .route("/", web::get().to(get_dashboard))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod pages;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, cookie::Key};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_files as fs;

use crate::pages::login::auth_user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            .service(fs::Files::new("/login", "./frontend").index_file("pages/login.html"))
            .route("/auth", web::post().to(auth_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

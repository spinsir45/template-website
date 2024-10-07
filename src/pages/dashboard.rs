use actix_web::{HttpRequest, HttpResponse};
use actix_session::Session;
use actix_files::NamedFile;
use std::path::PathBuf;

pub async fn get_dashboard(session: Session, req: HttpRequest) -> HttpResponse {
    if session.get::<String>("user").unwrap_or(None).is_some() {
        let path: PathBuf = "./frontend/pages/dashboard.html".parse().unwrap();
        match NamedFile::open(path) {
            Ok(file) => {
                println!("Made it to dashboard");
                file.into_response(&req)
            },
            Err(_) => {
                println!("Failed to get file");
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Found().append_header(("Location", "/login")).finish()
    }
}

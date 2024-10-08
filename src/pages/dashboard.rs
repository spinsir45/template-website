use actix_web::{HttpResponse, web};
use actix_session::Session;
use tera::{ Tera, Context };

pub async fn get_dashboard(session: Session, tmpl: web::Data<Tera>) -> HttpResponse {
    if session.get::<String>("user").unwrap_or(None).is_some() {
        let ctx = Context::new();
        let rendered = tmpl.render("dashboard.html", &ctx)
            .map_err(|_| HttpResponse::InternalServerError().body("Error rendering template"));

        match rendered {
            Ok(page) => {
                HttpResponse::Ok().content_type("text/html").body(page)
            },
            Err(e) => {
                e
            }
        }
    } else {
        HttpResponse::Found().append_header(("Location", "/login")).finish()
    }
}

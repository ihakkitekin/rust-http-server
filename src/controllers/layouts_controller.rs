use actix_web::{web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/layouts").route(web::get().to(|| HttpResponse::Ok().body("Layouts"))),
    );
}

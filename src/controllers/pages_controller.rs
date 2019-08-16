use actix_web::{web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/pages").route(web::get().to(|| HttpResponse::Ok().body("Pages"))));
}

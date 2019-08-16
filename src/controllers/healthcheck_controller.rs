use actix_web::{web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/liveness").route(web::get().to(|| HttpResponse::Ok().body("OK"))));
    cfg.service(web::resource("/readiness").route(web::get().to(|| HttpResponse::Ok().body("OK"))));
}

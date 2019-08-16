use actix_web::web;

use crate::handlers::fragments_handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/fragments").route(web::get().to(fragments_handler::get_fragments)));
}

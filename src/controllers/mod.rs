use actix_web::web;

mod fragments_controller;
mod healthcheck_controller;
mod layouts_controller;
mod pages_controller;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/monitoring").configure(healthcheck_controller::routes));
    cfg.service(
        web::scope("/api")
            .configure(fragments_controller::routes)
            .configure(layouts_controller::routes)
            .configure(pages_controller::routes),
    );
}

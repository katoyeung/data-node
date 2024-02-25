pub mod hello;
pub mod echo;
pub mod add;
pub mod search;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/", web::get().to(hello::greet))
            .route("/echo", web::post().to(echo::echo))
            .route("/add", web::post().to(add::add))
            .route("/search", web::post().to(search::search)),
    );
}

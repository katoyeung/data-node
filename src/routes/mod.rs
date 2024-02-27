pub mod add;
pub mod hello;
pub mod index;
pub mod search;
pub mod status;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/status", web::get().to(status::status_info))
            .route("/status/{index}", web::get().to(status::status_ft_info))
            .route("/add", web::post().to(add::add))
            .route("/search", web::post().to(search::search))
            .route("/index", web::post().to(index::index)),
    )
    .route("/", web::get().to(hello::greet)); // Moved this line out of the `.service()` call
}

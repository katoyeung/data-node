pub mod add;
pub mod delete;
pub mod hello;
pub mod index;
pub mod search;
pub mod status;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/status").route(web::get().to(status::status_info)))
        .service(web::resource("/status/{index}").route(web::get().to(status::status_ft_info)))
        .service(web::resource("/add").route(web::post().to(add::add)))
        .service(web::resource("/search").route(web::post().to(search::search)))
        .service(web::resource("/index").route(web::post().to(index::index)))
        .service(web::resource("/delete").route(web::post().to(delete::delete)))
        .route("/", web::get().to(hello::greet));
}

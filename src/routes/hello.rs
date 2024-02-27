use actix_web::{HttpResponse, Responder};
use chrono::Local;

pub async fn greet() -> impl Responder {
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    HttpResponse::Ok().body(format!("Hello World! {}", current_time))
}

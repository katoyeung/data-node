use actix_web::HttpResponse;

pub async fn greet() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

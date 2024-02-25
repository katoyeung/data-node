use actix_web::HttpResponse;

// Echoes back the input received in the request body
pub async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}

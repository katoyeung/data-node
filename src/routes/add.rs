use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn add(
    req_body: web::Json<serde_json::Value>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match app_state.redis_service.add(req_body.into_inner()).await {
        Ok(key) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error add item: {}", e)),
    }
}

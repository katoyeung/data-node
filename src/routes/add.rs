use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn add(
    req_body: web::Json<Vec<serde_json::Value>>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match app_state.redis_service.add(req_body.into_inner()).await {
        Ok(key) => HttpResponse::Ok().json(key),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("[Add] {}", e)
        })),
    }
}

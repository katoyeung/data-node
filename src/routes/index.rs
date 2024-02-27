use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn index(
    req_body: web::Json<serde_json::Value>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match app_state.redis_service.index(req_body.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("[Index] {}", e)
        })),
    }
}

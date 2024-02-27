use crate::AppState;
use actix_web::{web, HttpResponse};

pub async fn status_info(app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.redis_service.status().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("[Info] {}", e)
        })),
    }
}

pub async fn status_ft_info(
    path: web::Path<String>, // Use `web::Path` to extract path parameters
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let index = path.into_inner(); // Extract the index name from the path
    match app_state.redis_service.ft_status(index).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("[FT.INFO] {}", e)
        })),
    }
}

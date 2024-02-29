use crate::models::delete_request::DeleteRequest;
use crate::AppState;
use actix_web::{web, HttpResponse};

pub async fn delete(req: web::Json<DeleteRequest>, app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.redis_service.delete(req.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("[Search] {}", e)
        })),
    }
}

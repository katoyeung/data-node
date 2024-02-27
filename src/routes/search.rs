use crate::models::search_request::SearchRequest;
use crate::AppState;
use actix_web::{web, HttpResponse};

pub async fn search(
    search_query: web::Json<SearchRequest>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    match app_state
        .redis_service
        .search(search_query.into_inner())
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("[Search] {}", e)
        })),
    }
}

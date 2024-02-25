mod config;
mod models;
mod routes;
mod services;
mod utils;

use crate::config::redis_config::create_redis_pool;
use crate::services::redis_service::RedisService;
use actix_web::{web, App, HttpServer};
use std::env;

pub struct AppState {
    pub redis_service: RedisService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dotenv::dotenv().ok();

    let redis_pool = create_redis_pool().await; // Create the pool
    let redis_service = RedisService::new(redis_pool);
    let app_data = web::Data::new(AppState { redis_service });

    // Load IP address and port from environment variables
    let server_ip = std::env::var("SERVER_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_binding = format!("{}:{}", server_ip, server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(routes::config)
    })
    .bind(&server_binding)?
    .run()
    .await
}

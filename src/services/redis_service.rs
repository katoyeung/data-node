use crate::models::search_request::SearchRequest;
use crate::utils::date_utils::parse_date_time_with_timezone;
use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

pub struct RedisService {
    pool: Arc<Pool<RedisConnectionManager>>,
}

impl RedisService {
    pub fn new(pool: Arc<Pool<RedisConnectionManager>>) -> Self {
        RedisService { pool }
    }

    pub async fn add(&self, mut data: Value) -> Result<String, Box<dyn std::error::Error>> {
        // Validate the presence and content of the "source" field
        if let Some(source) = data.get("source").and_then(|s| s.as_str()) {
            if source.trim().is_empty() {
                // Return an error response if "source" is empty
                return Err("The 'source' field is missing or empty.".into());
            }
        } else {
            // Return an error if "source" is missing
            return Err("The 'source' field is required.".into());
        }

        let mut con = self.pool.get().await?;

        let source = data["source"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .replace(" ", "_");
        let key = format!("{}:{}", source, Uuid::new_v4());
        let created_at = chrono::Utc::now().to_rfc3339();
        let created_ts = chrono::Utc::now().timestamp();

        data["key"] = json!(key);
        data["created_at"] = json!(created_at);
        data["created_ts"] = json!(created_ts);

        // Convert the modified JSON back to a string
        let json_str = serde_json::to_string(&data).unwrap();

        // Store the JSON in Redis using RedisJSON command
        let _: () = redis::cmd("JSON.SET")
            .arg(&key)
            .arg("$")
            .arg(&json_str)
            .query_async(&mut *con)
            .await?;

        Ok(key)
    }

    pub async fn search(&self, req: SearchRequest) -> Result<Value, Box<dyn std::error::Error>> {
        let process_start_time = Instant::now();

        let index_name = &req.source;
        let query = req.q.unwrap_or_else(|| "*".to_string());
        let offset = req.offset.unwrap_or(0);
        let limit = req.limit.unwrap_or(20);

        // Initialize query string conditionally
        let mut query_str = if query != "*" {
            format!("@post_title|post_message:({})", query)
        } else {
            "".to_string()
        };

        // Check if start and end times are provided and valid
        if let (Some(start_time_str), Some(end_time_str)) = (&req.start_time, &req.end_time) {
            // Use the parse_date_time_with_timezone function directly
            let start_time = parse_date_time_with_timezone(start_time_str, 8);
            let end_time = parse_date_time_with_timezone(end_time_str, 8);

            let start_time_timestamp = start_time.timestamp();
            let end_time_timestamp = end_time.timestamp();

            let space_if_needed = if !query_str.is_empty() { " " } else { "" };
            query_str = format!(
                "{}{}@post_timestamp:[{},{}]",
                query_str, space_if_needed, start_time_timestamp, end_time_timestamp
            );
        }

        // Obtain a connection from the pool
        let mut con = self.pool.get().await?;

        let command_str = format!(
            "FT.SEARCH {} \"{}\" LIMIT {} {}",
            index_name, query_str, offset, limit
        );
        println!("Executing Redis command: {}", command_str);

        let raw_search_results: Vec<redis::Value> = redis::cmd("FT.SEARCH")
            .arg(&index_name)
            .arg(&query_str)
            .arg("LIMIT")
            .arg(offset.to_string())
            .arg(limit.to_string())
            .query_async(&mut *con)
            .await?;

        // Extract total_hits from the first element of the response
        let total_hits = match raw_search_results.get(0) {
            Some(redis::Value::Int(total)) => *total as u32,
            _ => 0,
        };

        // Process the rest of the response to extract document data
        let documents = raw_search_results
            .into_iter()
            .enumerate()
            .filter_map(|(index, value)| {
                // Skip the first element (total hits) and every odd element (document ID),
                // so we start processing from the second element (index 1) and take every other element (the JSON content).
                if index >= 2 && index % 2 == 0 {
                    // Adjusted to process correctly
                    match value {
                        redis::Value::Bulk(items) => {
                            items.get(1).and_then(|data| {
                                match data {
                                    redis::Value::Data(bytes) => {
                                        // Parse the JSON data into a serde_json::Value
                                        serde_json::from_slice::<Value>(bytes).ok()
                                    }
                                    _ => None,
                                }
                            })
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Value>>();

        let processing_time_ms = process_start_time.elapsed().as_millis();
        let page = offset / limit + 1;
        let total_pages = (total_hits + limit as u32 - 1) / limit as u32;

        let response = json!({
            "data": documents,
            "query": &query,
            "totals": total_hits,
            "processingTimeMs": processing_time_ms,
            "perPage": limit,
            "page": page,
            "totalPages": total_pages
        });

        Ok(response)
    }
}

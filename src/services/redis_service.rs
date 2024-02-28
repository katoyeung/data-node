use crate::models::search_request::SearchRequest;
use crate::utils::date_utils::parse_date_time_with_timezone;
use log::debug;
use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

pub struct RedisService {
    pool: Arc<Pool<RedisConnectionManager>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexRequest {
    index_name: String,
    #[serde(rename = "type")]
    index_type: String,
    language: Option<String>,
    prefixes: Vec<String>,
    schema: Vec<SchemaField>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaField {
    field_name: String,
    field_type: String,
    sortable: bool,
}

impl RedisService {
    pub fn new(pool: Arc<Pool<RedisConnectionManager>>) -> Self {
        RedisService { pool }
    }

    pub async fn status(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut con = self.pool.get().await?;
        let response: String = redis::cmd("INFO").query_async(&mut *con).await?;

        // Split the response into lines and then iterate over each line
        let lines = response.lines();

        let mut info_map: HashMap<&str, &str> = HashMap::new();

        for line in lines {
            // Ignore comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            // Split each line by the first colon to separate the key and value
            if let Some((key, value)) = line.split_once(':') {
                info_map.insert(key, value);
            }
        }

        // Convert the HashMap into a JSON Value
        let json = serde_json::to_value(info_map)?;

        Ok(json)
    }

    pub async fn ft_status(&self, index: String) -> Result<Value, Box<dyn Error>> {
        let mut con = self.pool.get().await?;
        let response: Vec<redis::Value> = redis::cmd("FT.INFO")
            .arg(&index)
            .query_async(&mut *con)
            .await?;

        // Convert Vec<redis::Value> to Vec<serde_json::Value>, filtering out nulls
        let json_response = response
            .into_iter()
            .filter_map(|val| {
                // Use filter_map to both map and filter out None values
                match val {
                    redis::Value::Nil => None, // Skip nil values
                    redis::Value::Int(i) => Some(json!(i)),
                    redis::Value::Data(vec) => {
                        serde_json::to_value(String::from_utf8(vec).ok()).ok()
                    }
                    redis::Value::Bulk(vec) => Some(json!(vec
                        .into_iter()
                        .filter_map(|v| match v {
                            redis::Value::Data(data) =>
                                serde_json::to_value(String::from_utf8(data).ok()).ok(),
                            // Skip null values, add cases for other types if necessary
                            _ => None,
                        })
                        .collect::<Vec<Value>>())),
                    redis::Value::Status(s) => Some(json!(s)),
                    // Implement other conversions as needed, skipping nulls
                    _ => None,
                }
            })
            .collect::<Vec<Value>>();

        // Convert Vec<serde_json::Value> to serde_json::Value (JSON Array), already filtered
        let json_array = Value::Array(json_response);

        Ok(json_array)
    }

    pub async fn index(&self, data: Value) -> Result<Value, Box<dyn std::error::Error>> {
        // Deserialize JSON data to IndexRequest struct
        let request: IndexRequest = serde_json::from_value(data)?;

        // Check for missing mandatory fields
        if request.index_name.is_empty() {
            return Err("index_name is missing".into());
        }

        if request.index_type.is_empty() {
            return Err("index_type is missing".into());
        }

        if request.prefixes.is_empty() {
            return Err("prefixes are missing".into());
        }

        if request.schema.is_empty() {
            return Err("schema is missing".into());
        }

        // Start building the Redis command
        let mut command = format!(
            "FT.CREATE {} ON {} PREFIX {} ",
            request.index_name,
            request.index_type,
            request.prefixes.len()
        );

        // Add prefixes
        for prefix in request.prefixes {
            command.push_str(&format!("{} ", prefix));
        }

        // Specify language if present
        if let Some(language) = request.language {
            command.push_str(&format!("LANGUAGE {} ", language));
        }

        // Add schema
        command.push_str("SCHEMA ");
        for field in request.schema {
            command.push_str(&format!("{} {} ", field.field_name, field.field_type));
            if field.sortable {
                command.push_str("SORTABLE ");
            }
        }

        // Trim the trailing space
        let redis_command = command.trim_end().to_string();
        debug!("Executing Redis command: {}", redis_command);

        // Splitting the command string into a vector of words for individual arguments
        let command_args: Vec<&str> = redis_command.split_whitespace().collect();

        let mut con = self.pool.get().await?;

        // Attempt to drop the existing index if it exists
        let command_drop = format!("FT.DROPINDEX {}", request.index_name);
        let command_drop_args: Vec<&str> = command_drop.split_whitespace().collect();
        let drop_response: Result<String, redis::RedisError> = redis::cmd(command_drop_args[0]) // This is "FT.DROPINDEX"
            .arg(&command_drop_args[1..]) // The rest of the arguments
            .query_async(&mut *con)
            .await;

        match drop_response {
            Ok(_) => debug!(
                "Existing index '{}' was dropped successfully.",
                request.index_name
            ),
            Err(e) => debug!(
                "No existing index to drop for '{}'. Error: {}",
                request.index_name, e
            ),
        }

        let response: String = redis::cmd(command_args[0]) // This is "FT.CREATE"
            .arg(&command_args[1..]) // The rest of the arguments
            .query_async(&mut *con)
            .await?;

        if response == "OK" {
            Ok(
                json!({"status": "success", "message": format!("Index '{}' created successfully.", request.index_name)}),
            )
        } else {
            Err(format!(
                "Failed to create index '{}': {}",
                request.index_name, response
            )
            .into())
        }
    }

    pub async fn add(&self, mut data: Value) -> Result<Value, Box<dyn std::error::Error>> {
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

        Ok(json!({"status": "success", "key": key}))
    }

    pub async fn search(&self, req: SearchRequest) -> Result<Value, Box<dyn std::error::Error>> {
        let process_start_time = Instant::now();

        let index_name = req.index.ok_or("The 'index' field is required.")?;

        let query = req.q.unwrap_or_else(|| "*".to_string());
        let offset = req.offset.unwrap_or(0); // Convert to string for command args
        let limit = req.limit.unwrap_or(10); // Convert to string for command args
        let language = req.language.unwrap_or_else(|| "chinese".to_string());
        let filter_date_field = req
            .filter_date_by
            .unwrap_or_else(|| "post_timestamp".to_string());

        let query_str = if !query.is_empty() && query != "*" {
            query.clone()
        } else {
            "*".to_string() // Default to "*" if the query is empty
        };

        // Sort parameters
        let sortby_field = req.sort_by.unwrap_or_else(|| "post_timestamp".to_string());
        let sort_order = req.sort_order.unwrap_or_else(|| "DESC".to_string());
        let offset_str = offset.to_string();
        let limit_str = limit.to_string();

        let filter_str =
            if let (Some(start_time_str), Some(end_time_str)) = (&req.start_time, &req.end_time) {
                let start_time = parse_date_time_with_timezone(start_time_str, 8);
                let end_time = parse_date_time_with_timezone(end_time_str, 8);

                vec![
                    "FILTER".to_string(),
                    filter_date_field,
                    start_time.timestamp().to_string(),
                    end_time.timestamp().to_string(),
                ]
            } else {
                Vec::new() // No filter if no start and end times
            };

        // Prepare command arguments with conditional inclusion of filter
        let mut command_args = vec![
            index_name.as_str(),
            query_str.as_str(),
            "LIMIT",
            &offset_str,
            &limit_str,
            "LANGUAGE",
            &language,
            "SORTBY",
            &sortby_field,
            &sort_order,
        ];

        if !filter_str.is_empty() {
            // Convert Vec<String> to Vec<&str> to match types
            let filter_str_refs: Vec<&str> = filter_str.iter().map(AsRef::as_ref).collect();
            command_args.extend_from_slice(&filter_str_refs);
        }

        // Log the command for debugging
        debug!(
            "Executing Redis command: FT.SEARCH {}",
            command_args.join(" ")
        );

        // Obtain a connection from the pool
        let mut con = self.pool.get().await?;

        let raw_search_results: Vec<redis::Value> = redis::cmd("FT.SEARCH")
            .arg(&command_args[..]) // Pass the arguments as a slice
            .query_async(&mut *con)
            .await?;

        // Extract total_hits from the first element of the response
        let total_hits = match raw_search_results.get(0) {
            Some(redis::Value::Int(total)) => *total as u32,
            _ => 0,
        };

        // Only process the rest of the response if there are hits
        let documents = if total_hits > 0 {
            raw_search_results
                .into_iter()
                .enumerate()
                .filter_map(|(index, value)| {
                    if index >= 2 && index % 2 == 0 {
                        match value {
                            redis::Value::Bulk(items) => items.get(3).and_then(|data| match data {
                                redis::Value::Data(bytes) => {
                                    serde_json::from_slice::<Value>(bytes).ok()
                                }
                                _ => None,
                            }),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<Value>>()
        } else {
            Vec::new()
        };

        let processing_time_ms = process_start_time.elapsed().as_millis();
        let page = offset / limit + 1;
        let total_pages = (total_hits + limit as u32 - 1) / limit as u32;

        let response = json!({
            "data": documents,
            "query": &query,
            "totals": total_hits,
            "processing_time_ms": processing_time_ms,
            "limit": limit,
            "offset": offset,
            "page": page,
            "totalPages": total_pages
        });

        Ok(response)
    }
}

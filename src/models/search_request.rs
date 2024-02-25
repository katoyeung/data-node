use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub source: String,
    pub q: Option<String>,          // Query string
    pub offset: Option<i64>,        // Pagination offset
    pub limit: Option<i64>,         // Number of items to retrieve
    pub start_time: Option<String>, // Optional start time filter
    pub end_time: Option<String>,   // Optional end time filter
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub source: Option<String>,
    pub q: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,

    pub language: Option<String>,
    pub filter_date_by: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

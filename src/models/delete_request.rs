use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRequest {
    pub source: Option<String>,
    pub keys: Option<Vec<String>>,
}

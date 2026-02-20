use serde::{Deserialize, Serialize};

use crate::models::{Address, Links};

/// Top-level API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_hits: Option<i64>,
}

use serde::{Deserialize, Serialize};

/// Reusable links object (HATEOAS).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<SelfLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

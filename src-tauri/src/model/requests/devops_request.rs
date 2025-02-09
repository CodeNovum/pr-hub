use crate::model::core::organization::Organization;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src/bindings/requests/")]
pub struct DevOpsRequest {
    pub organization: Organization,
    #[ts(rename = "repositories")]
    #[serde(rename = "repositories")]
    pub repositories: Vec<Option<(String, String)>>,
}

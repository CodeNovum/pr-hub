use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct DevOpsUser {
    pub id: Option<String>,
    #[ts(rename = "displayName")]
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

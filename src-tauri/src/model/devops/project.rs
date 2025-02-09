use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct Project {
    pub name: String,
    #[ts(rename = "organizationName")]
    #[serde(rename = "organizationName")]
    pub organization_name: Option<String>,
}

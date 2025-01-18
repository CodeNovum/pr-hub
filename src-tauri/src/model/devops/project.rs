use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct Project {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub state: Option<String>,
    pub revision: Option<i32>,
    pub visibility: Option<String>,
    #[ts(rename = "organizationName")]
    #[serde(rename = "organizationName")]
    pub organization_name: Option<String>,
    #[ts(rename = "isActive")]
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
}

use crate::model::devops::project::Project;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct GitRepository {
    pub id: Option<String>,
    pub name: Option<String>,
    pub project: Option<Project>,
    #[ts(rename = "isActive")]
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
}

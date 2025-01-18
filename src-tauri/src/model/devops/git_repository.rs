use crate::model::devops::project::Project;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct GitRepository {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub project: Option<Project>,
    #[ts(rename = "defaultBranch")]
    #[serde(rename = "defaultBranch")]
    pub default_branch: Option<String>,
    pub size: Option<i32>,
    #[ts(rename = "remoteUrl")]
    #[serde(rename = "remoteUrl")]
    pub remote_url: Option<String>,
    #[ts(rename = "sshUrl")]
    #[serde(rename = "sshUrl")]
    pub ssh_url: Option<String>,
    #[ts(rename = "webUrl")]
    #[serde(rename = "webUrl")]
    pub web_url: Option<String>,
    #[ts(rename = "isDisabled")]
    #[serde(rename = "isDisabled")]
    pub is_disabled: Option<bool>,
}

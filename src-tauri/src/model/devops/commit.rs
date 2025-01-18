use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct Commit {
    #[ts(rename = "commitId")]
    #[serde(rename = "commitId")]
    pub commit_id: Option<String>,
    pub url: Option<String>,
}

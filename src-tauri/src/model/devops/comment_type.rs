use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, PartialEq)]
#[ts(export, export_to = "../src/bindings/devops/")]
pub enum CommentType {
    #[serde(rename = "codeChange")]
    CodeChange,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "unknown")]
    Unknown,
}

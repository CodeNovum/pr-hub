use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, PartialEq)]
#[ts(export, export_to = "../src/bindings/devops/")]
pub enum PullRequestCommentThreadStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "byDesign")]
    ByDesign,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "wontFix")]
    WontFix,
}

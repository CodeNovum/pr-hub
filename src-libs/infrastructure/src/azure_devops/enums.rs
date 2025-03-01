use serde::{Deserialize, Serialize};

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-request-threads/list?view=azure-devops-rest-6.0&tabs=HTTP#commenttype
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum CommentType {
    #[serde(rename = "codeChange")]
    CodeChange,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "unknown")]
    #[default]
    Unknown,
}

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-request-threads/list?view=azure-devops-rest-6.0&tabs=HTTP#commentthreadstatus
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
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
    #[default]
    Unknown,
    #[serde(rename = "wontFix")]
    WontFix,
}

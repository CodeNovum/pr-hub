use crate::model::devops::{
    devops_user::DevOpsUser, git_repository::GitRepository, label::Label,
    pull_request_comment_thread::PullRequestCommentThread,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct PullRequest {
    pub repository: Option<GitRepository>,
    #[ts(rename = "pullRequestId")]
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: Option<i32>,
    pub status: Option<String>,
    #[ts(rename = "createdBy")]
    #[serde(rename = "createdBy")]
    pub created_by: Option<DevOpsUser>,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "creationDate")]
    #[serde(rename = "creationDate")]
    pub creation_date: Option<DateTime<Utc>>,
    pub title: Option<String>,
    #[ts(rename = "mergeStatus")]
    #[serde(rename = "mergeStatus")]
    pub merge_status: Option<String>,
    pub reviewers: Option<Vec<DevOpsUser>>,
    pub labels: Option<Vec<Label>>,
    #[ts(rename = "commentThreads")]
    #[serde(rename = "commentThreads")]
    pub comment_threads: Option<Vec<PullRequestCommentThread>>,
    #[serde(rename = "organizationName")]
    pub organization_name: Option<String>,
}

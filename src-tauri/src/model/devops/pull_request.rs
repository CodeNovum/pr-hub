use crate::model::devops::{
    commit::Commit, devops_user::DevOpsUser, git_repository::GitRepository, label::Label,
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
    #[ts(rename = "codeReviewId")]
    #[serde(rename = "codeReviewId")]
    pub code_review_id: Option<i32>,
    pub status: Option<String>,
    #[ts(rename = "createdBy")]
    #[serde(rename = "createdBy")]
    pub created_by: Option<DevOpsUser>,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "creationDate")]
    #[serde(rename = "creationDate")]
    pub creation_date: Option<DateTime<Utc>>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[ts(rename = "sourceRefName")]
    #[serde(rename = "sourceRefName")]
    pub source_ref_name: Option<String>,
    #[ts(rename = "targetRefName")]
    #[serde(rename = "targetRefName")]
    pub target_ref_name: Option<String>,
    #[ts(rename = "mergeStatus")]
    #[serde(rename = "mergeStatus")]
    pub merge_status: Option<String>,
    #[ts(rename = "isDraft")]
    #[serde(rename = "isDraft")]
    pub is_draft: Option<bool>,
    #[ts(rename = "mergeId")]
    #[serde(rename = "mergeId")]
    pub merge_id: Option<String>,
    #[ts(rename = "lastMergeSourceCommit")]
    #[serde(rename = "lastMergeSourceCommit")]
    pub last_merge_source_commit: Option<Commit>,
    #[ts(rename = "lastMergeTargetCommit")]
    #[serde(rename = "lastMergeTargetCommit")]
    pub last_merge_target_commit: Option<Commit>,
    #[ts(rename = "lastMergeCommit")]
    #[serde(rename = "lastMergeCommit")]
    pub last_merge_commit: Option<Commit>,
    pub reviewers: Option<Vec<DevOpsUser>>,
    pub labels: Option<Vec<Label>>,
    pub url: Option<String>,
    #[ts(rename = "supportsIterations")]
    #[serde(rename = "supportsIterations")]
    pub supports_iterations: Option<bool>,
    #[ts(rename = "commentThreads")]
    #[serde(rename = "commentThreads")]
    pub comment_threads: Option<Vec<PullRequestCommentThread>>,
    #[serde(rename = "organizationName")]
    pub organization_name: Option<String>,
}

use crate::model::devops::{
    comment::Comment, pull_request_comment_thread_status::PullRequestCommentThreadStatus,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct PullRequestCommentThread {
    pub id: Option<i32>,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "publishedDate")]
    #[serde(rename = "publishedDate")]
    pub published_date: Option<DateTime<Utc>>,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "lastUpdatedDate")]
    #[serde(rename = "lastUpdatedDate")]
    pub last_updated_date: Option<DateTime<Utc>>,
    pub comments: Option<Vec<Comment>>,
    pub status: Option<PullRequestCommentThreadStatus>,
}

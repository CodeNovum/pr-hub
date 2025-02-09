use crate::model::devops::{
    comment::Comment, pull_request_comment_thread_status::PullRequestCommentThreadStatus,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct PullRequestCommentThread {
    pub comments: Option<Vec<Comment>>,
    pub status: Option<PullRequestCommentThreadStatus>,
}

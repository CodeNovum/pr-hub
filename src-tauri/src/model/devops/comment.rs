use crate::model::devops::{comment_type::CommentType, devops_user::DevOpsUser, link::Link};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct Comment {
    pub id: Option<i32>,
    #[ts(rename = "parentCommentId")]
    #[serde(rename = "parentCommentId")]
    pub parent_comment_id: Option<i32>,
    pub author: Option<DevOpsUser>,
    pub content: Option<String>,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "publishedDate")]
    #[serde(rename = "publishedDate")]
    pub published_date: Option<DateTime<Utc>>,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "lastUpdatedDate")]
    #[serde(rename = "lastUpdatedDate")]
    pub last_updated_date: Option<DateTime<Utc>>,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "lastContentUpdatedDate")]
    #[serde(rename = "lastContentUpdatedDate")]
    pub last_content_updated_date: Option<DateTime<Utc>>,
    #[ts(rename = "commentType")]
    #[serde(rename = "commentType")]
    pub comment_type: Option<CommentType>,
    #[ts(rename = "usersLiked")]
    #[serde(rename = "usersLiked")]
    pub users_liked: Option<Vec<DevOpsUser>>,
    pub links: Option<HashMap<String, Link>>,
}

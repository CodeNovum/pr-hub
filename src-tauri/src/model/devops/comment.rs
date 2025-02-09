use crate::model::devops::comment_type::CommentType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct Comment {
    #[ts(rename = "commentType")]
    #[serde(rename = "commentType")]
    pub comment_type: Option<CommentType>,
}

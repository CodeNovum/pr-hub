use crate::model::devops::link::Link;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct DevOpsUser {
    pub url: Option<String>,
    pub id: Option<String>,
    #[ts(rename = "uniqueName")]
    #[serde(rename = "uniqueName")]
    pub unique_name: Option<String>,
    #[ts(rename = "imageUrl")]
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[ts(rename = "directoryAlias")]
    #[serde(rename = "directoryAlias")]
    pub directory_alias: Option<String>,
    pub domain: Option<String>,
    #[ts(rename = "principalName")]
    #[serde(rename = "principalName")]
    pub principal_name: Option<String>,
    #[ts(rename = "mailAddress")]
    #[serde(rename = "mailAddress")]
    pub mail_address: Option<String>,
    pub origin: Option<String>,
    #[ts(rename = "originId")]
    #[serde(rename = "originId")]
    pub origin_id: Option<String>,
    #[ts(rename = "displayName")]
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub links: Option<HashMap<String, Link>>,
    pub descriptor: Option<String>,
    #[ts(rename = "isFlagged")]
    #[serde(rename = "isFlagged")]
    pub is_flagged: Option<bool>,
    #[ts(rename = "isRequired")]
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[ts(rename = "hasDeclined")]
    #[serde(rename = "hasDeclined")]
    pub has_declined: Option<bool>,
    pub vote: Option<i32>,
    #[ts(rename = "reviewerUrl")]
    #[serde(rename = "reviewerUrl")]
    pub reviewer_url: Option<String>,
}

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src/bindings/core/")]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub pat: String,
    #[ts(rename = "isPatValid")]
    #[serde(rename = "isPatValid")]
    pub is_pat_valid: bool,
}

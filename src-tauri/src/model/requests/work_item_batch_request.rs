use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/requests/")]
pub struct WorkItemBatchRequest {
    pub ids: Vec<i32>,
    pub fields: Vec<String>,
}

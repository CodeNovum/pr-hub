use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Default)]
#[ts(export, export_to = "../src/bindings/devops/")]
#[serde(default)]
pub struct Label {
    pub id: Option<String>,
    pub name: Option<String>,
}

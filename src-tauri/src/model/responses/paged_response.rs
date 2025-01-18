use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/responses/")]
struct PagedResponse<T> {
    #[ts(rename = "itemsAvailable")]
    #[serde(rename = "itemsAvailable")]
    pub items_available: i32,
    #[ts(rename = "itemsPerPage")]
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i32,
    #[ts(rename = "itemsTotal")]
    #[serde(rename = "itemsTotal")]
    pub items_total: i32,
    pub page: i32,
    #[ts(rename = "pagesAvailable")]
    #[serde(rename = "pagesAvailable")]
    pub pages_available: i32,
    pub results: Vec<T>,
}

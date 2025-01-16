use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/requests/")]
pub struct PagedRequest {
    #[ts(rename = "filterText")]
    #[serde(rename = "filterText")]
    pub filter_text: Option<String>,
    #[ts(rename = "itemsPerPage")]
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: Option<i32>,
    #[ts(rename = "pageToDeliver")]
    #[serde(rename = "pageToDeliver")]
    pub page_to_deliver: Option<i32>,
    #[ts(rename = "sortDirection")]
    #[serde(rename = "sortDirection")]
    pub sort_direction: Option<SortDirection>,
    #[ts(type = "string | null | undefined")]
    #[ts(rename = "sortPropertyName")]
    #[serde(rename = "sortPropertyName")]
    pub sort_property_name: Option<String>,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/requests/")]
pub enum SortDirection {
    Undefined,
    Ascending,
    Descending,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PagedResponse<T: ToSchema> {
    pub data: Vec<T>,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    #[serde(rename = "currentPage")]
    pub current_page: i64,
    pub limit: i64,
}

impl<T: ToSchema> PagedResponse<T> {
    pub fn new(
        data: Vec<T>,
        total_items: i64,
        total_pages: i64,
        current_page: i64,
        limit: i64,
    ) -> Self {
        Self {
            data,
            total_items,
            total_pages,
            current_page,
            limit,
        }
    }
}

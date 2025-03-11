use chrono::NaiveDateTime;
use sqlx::types::chrono;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Files {
    pub file_id: i32,
    pub file_blob: Vec<u8>,
    pub size: i32,
    pub file_type: String,
    pub account_id: i32,
    pub created_at: NaiveDateTime
}
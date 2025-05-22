use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, FromRow, Debug, ToSchema, Clone)]
pub struct Part {
    pub id: i32,
    pub car_id: Option<i32>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct NewPart {
    pub car_id: i32,
    #[validate(length(min = 1, max = 80))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PartQuery {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct PartList {
    pub data: Vec<Part>,
    pub total: i64,
}

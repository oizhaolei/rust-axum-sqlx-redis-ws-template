use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, ToSchema)]
pub struct Part {
    pub id: i32,
    pub car_id: Option<i32>,
    pub name: String,
}

pub type PartList = Vec<Part>;

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct NewPart {
    pub car_id: i32,
    #[validate(length(min = 1, max = 80))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartQuery {
    pub name: Option<String>,
}

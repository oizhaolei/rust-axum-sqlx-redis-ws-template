use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, FromRow, Debug, ToSchema, Clone)]
pub struct Car {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub year: Option<i16>,
}

pub type CarList = Vec<Car>;

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct NewCar {
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    pub color: Option<String>,
    pub year: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CarQuery {
    pub name: Option<String>,
}

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, FromRow, Debug, ToSchema, Clone, Validate)]
pub struct User {
    #[validate(length(min = 3, max = 16),regex(path = *USERNAME_REGEX))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password_hash: String,
}

pub type UserList = Vec<User>;

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct NewUser {
    #[validate(length(min = 3, max = 16),regex(path = *USERNAME_REGEX))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserQuery {
    pub username: Option<String>,
}

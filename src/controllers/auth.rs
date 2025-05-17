use crate::error::{AppError, AppJson};
use crate::models::user::UserAuth;
use crate::repositories::UserRepoExt;
use crate::router::AUTH_TAG;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use std::sync::LazyLock;
use utoipa::ToSchema;
use validator::Validate;

use crate::services;
use axum::{
    Json, RequestPartsExt,
    extract::{Extension, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

/// Test Auth
///
/// Tries to login via a User in the database.
#[utoipa::path(
    post,
    path = "/test",
    tag = AUTH_TAG,
    security(
        ("bearerAuth" = [])
    ),
    responses(
        (status = 200, description = "User login successfully", body = String)
    )
)]
pub async fn test(claims: Claims) -> Result<String, AppError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

/// Authorize with username and password
///
/// Tries to login via a User in the database.
#[utoipa::path(
        post,
        path = "/authorize",
        tag = AUTH_TAG,
        request_body(content=UserAuth, content_type="application/json", description="authorize"),
        responses(
            (status = 200, description = "User login successfully", body = AuthBody)
        )
)]
pub async fn authorize(
    Extension(repo): UserRepoExt,
    Json(user): Json<UserAuth>,
) -> Result<AppJson<AuthBody>, AppError> {
    let user = services::users::login(repo.clone(), &user).await?;

    let claims = Claims {
        sub: user.username,
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)?;

    // Send the authorized token
    Ok(AppJson(AuthBody::new(token)))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

// #[derive(Debug, Deserialize)]
// struct AuthPayload {
//     username: String,
//     password: String,
// }

#[allow(dead_code)]
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

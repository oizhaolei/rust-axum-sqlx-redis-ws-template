use crate::error::{AppError, AppJson};
use crate::models::user::{User, UserAuth, UserList, UserQuery};
use crate::repositories::UserRepoExt;
use crate::router::USERS_TAG;
use crate::services;
use axum::{
    Json,
    extract::{Extension, Path},
};
use axum_extra::extract::Query;

use super::auth::Claims;
use super::{CommonQuery, Pagination};

/// List Users
///
/// Tries to all Users from the database.
#[utoipa::path(
    get,
    path = "/list",
    params(
        ("name" = inline(Option<String>), Query, description="User Name"),
        ("ids" = inline(Option<String>), Query, description="ids"),
        ("page" = inline(Option<usize>), Query, description="Page"),
        ("perPage" = inline(Option<usize>), Query, description="PerPage"),
        ("field" = inline(Option<String>), Query, description="Field"),
        ("order" = inline(Option<String>), Query, description="Order")
    ) ,
    responses((status = OK, body = UserList)),
    tag = USERS_TAG
)]
pub async fn list(
    Query(conditions): Query<UserQuery>,
    Query(query): Query<CommonQuery>,
    Query(pagination): Query<Pagination>,
    Extension(repo): UserRepoExt,
) -> Result<AppJson<UserList>, AppError> {
    println!("list params: {:?}", pagination);
    println!("conditions: {:?}", conditions);
    println!("ids: {:?}", query);
    let users = services::users::find_all(repo.clone(), &conditions, &query, &pagination).await?;
    Ok(AppJson(users))
}

/// Get single User by username
///
/// Tries to get single user by username from the database
#[utoipa::path(
    get,
    path = "/{username}",
    params(("username" =&str, Path, description="User Id")),
    responses((status = OK, body = User)),
    security(
        ("bearerAuth" = [])
    ),
    tag = USERS_TAG
)]
pub async fn view(
    _claims: Claims,
    Path(username): Path<String>,
    Extension(repo): UserRepoExt,
) -> Result<AppJson<User>, AppError> {
    let user = services::users::view(repo.clone(), &username).await?;
    Ok(AppJson(user))
}

/// Create new User
///
/// Tries to create a new User in the database.
#[utoipa::path(
        post,
        path = "/create",
        security(
            ("bearerAuth" = [])
        ),
        tag = USERS_TAG,
        request_body(content=UserAuth, content_type="application/json", description="New User Information"),
        responses(
            (status = 201, description = "User item created successfully", body = User)
        )
)]
pub async fn create(
    _claims: Claims,
    Extension(repo): UserRepoExt,
    Json(new_user): Json<UserAuth>,
) -> Result<AppJson<User>, AppError> {
    let user = services::users::create(repo.clone(), &new_user).await?;
    Ok(AppJson(user))
}

/// Update existing User
///
/// Tries to update a User in the database.
#[utoipa::path(
        post,
        path = "/update",
        security(
            ("bearerAuth" = [])
        ),
        tag = USERS_TAG,
        request_body(content=User, content_type="application/json", description="User To Update"),
        responses(
            (status = 200, description = "User item updated successfully", body = User)
        )
)]
pub async fn update(
    _claims: Claims,
    Extension(repo): UserRepoExt,
    Json(user): Json<UserAuth>,
) -> Result<AppJson<User>, AppError> {
    let user = services::users::update(repo.clone(), &user).await?;
    Ok(AppJson(user))
}

/// Delete existing User
///
/// Tries to delete a User from the database.
#[utoipa::path(
        delete,
        path = "/delete/{username}",
        params(("username" = String, Path, description="User Id")),
        security(
            ("bearerAuth" = [])
        ),
        tag = USERS_TAG,
        responses(
            (status = 200, description = "User item deleted successfully", body = String)
        )
)]
pub async fn delete(
    _claims: Claims,
    Path(username): Path<String>,
    Extension(repo): UserRepoExt,
) -> Result<(), AppError> {
    services::users::delete(repo.clone(), &username).await?;
    Ok(())
}

// Example of end-to-end test with real database and repository
// 1. run `docker-compose -f compose-tests.yaml up -d` to start up the test db server
// 2. remove #[ignore] on the test method
#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::controllers::users;
    use crate::models::user::{UserAuth, UserList};
    use crate::repositories::user::UserRepository;
    use crate::repositories::{clear_database, create_user_repository, run_migrations};
    use axum::http::Request;
    use axum::routing::get;
    use axum::{Extension, Router, body::Body, http::StatusCode};
    use once_cell::sync::Lazy;
    use std::sync::Arc;
    use tower::ServiceExt;

    static INIT: Lazy<()> = Lazy::new(|| {
        dotenv::from_filename(".env.test").ok();
        println!("Test environment loaded");
    });

    #[tokio::test]
    #[ignore]
    async fn list() {
        Lazy::force(&INIT);
        let config = Config::init();
        let _ = run_migrations(&config).await;
        let _ = clear_database(&config).await;
        let real_repo = create_user_repository(&config).await;

        // given
        let user = UserAuth {
            username: "Tesla".to_string(),
            password: "Red".to_string(),
        };
        real_repo.create(&user).await.unwrap();

        // Create an Axum router with the mock repository as an extension
        let app = Router::new()
            .route("/users", get(users::list))
            .layer(Extension(Arc::new(real_repo)));

        // Build a request to simulate a GET /users
        let request = Request::builder()
            .uri("/users?name=Tesla")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        // Use `oneshot` to send a single request through the router
        let service = tower::ServiceBuilder::new().service(app);
        // when
        let response = service
            .oneshot(request)
            .await
            .expect("Failed to execute request");

        // then
        // Check the response status code
        assert_eq!(response.status(), StatusCode::OK);
        let max_body_size = 10 * 1024;
        let response_body = axum::body::to_bytes(response.into_body(), max_body_size)
            .await
            .expect("Failed to read body");
        let users: UserList =
            serde_json::from_slice(&response_body).expect("Failed to deserialize response");
        assert_eq!(users.data[0].username, "Tesla");
        assert_eq!(users.data[0].password_hash, "Red".to_string());
    }
}

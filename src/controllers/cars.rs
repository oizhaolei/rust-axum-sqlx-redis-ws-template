use crate::cache::CacheExt;
use crate::error::{AppError, AppJson};
use crate::models::car::{Car, CarList, CarQuery, NewCar};
use crate::repositories::CarRepoExt;
use crate::router::CARS_TAG;
use crate::services;
use axum::{
    Json,
    extract::{Extension, Path},
};
use axum_extra::extract::Query;

use super::auth::Claims;
use super::{CommonQuery, Pagination};

/// List Cars
///
/// Tries to all Cars from the database.
#[utoipa::path(
    get,
    path = "/list",
    params(
        ("name" = inline(Option<String>), Query, description="Car Name"),
        ("ids" = inline(Option<String>), Query, description="ids"),
        ("page" = inline(Option<usize>), Query, description="Page"),
        ("perPage" = inline(Option<usize>), Query, description="PerPage"),
        ("field" = inline(Option<String>), Query, description="Field"),
        ("order" = inline(Option<String>), Query, description="Order")
    ) ,
    responses((status = OK, body = [Car])),
    tag = CARS_TAG
)]
pub async fn list(
    Query(conditions): Query<CarQuery>,
    Query(query): Query<CommonQuery>,
    Query(pagination): Query<Pagination>,
    Extension(repo): CarRepoExt,
) -> Result<AppJson<CarList>, AppError> {
    println!("list params: {:?}", pagination);
    println!("conditions: {:?}", conditions);
    println!("ids: {:?}", query);
    let cars = services::cars::search(repo.clone(), &conditions, &query, &pagination).await?;
    Ok(AppJson(cars))
}

///
/// Tries to get single car by id from the database
#[utoipa::path(
    get,
    path = "/{car_id}",
    params(("car_id" = i32, Path, description="Car Id")),
    responses((status = OK, body = [Car])),
    tag = CARS_TAG
)]
pub async fn view(
    Path(car_id): Path<i32>,
    Extension(repo): CarRepoExt,
    Extension(cache): CacheExt,
) -> Result<AppJson<Car>, AppError> {
    let car = services::cars::view(repo.clone(), cache.clone(), car_id).await?;
    Ok(AppJson(car))
}

/// Create new Car
///
/// Tries to create a new Car in the database.
#[utoipa::path(
        post,
        path = "/create",
        tag = CARS_TAG,
        security(
            ("bearerAuth" = [])
        ),
        request_body(content=NewCar, content_type="application/json", description="New Car Information"),
        responses(
            (status = 201, description = "Car item created successfully", body = Car)
        )
)]
pub async fn create(
    _claims: Claims,
    Extension(repo): CarRepoExt,
    Json(new_car): Json<NewCar>,
) -> Result<AppJson<Car>, AppError> {
    let car = services::cars::create(repo.clone(), &new_car).await?;
    Ok(AppJson(car))
}

/// Update existing Car
///
/// Tries to update a Car in the database.
#[utoipa::path(
        post,
        path = "/update",
        tag = CARS_TAG,
        security(
            ("bearerAuth" = [])
        ),
        request_body(content=Car, content_type="application/json", description="Car To Update"),
        responses(
            (status = 200, description = "Car item updated successfully", body = Car)
        )
)]
pub async fn update(
    _claims: Claims,
    Extension(repo): CarRepoExt,
    Extension(cache): CacheExt,
    Json(car): Json<Car>,
) -> Result<AppJson<Car>, AppError> {
    let car = services::cars::update(repo.clone(), cache, &car).await?;
    Ok(AppJson(car))
}

/// Delete existing Car
///
/// Tries to delete a Car from the database.
#[utoipa::path(
        delete,
        path = "/delete/{car_id}",
        params(("car_id" = i32, Path, description="Car Id")),
        tag = CARS_TAG,
        security(
            ("bearerAuth" = [])
        ),
        responses(
            (status = 200, description = "Car item deleted successfully", body = String)
        )
)]
pub async fn delete(
    _claims: Claims,
    Path(car_id): Path<i32>,
    Extension(repo): CarRepoExt,
    Extension(cache): CacheExt,
) -> Result<(), AppError> {
    services::cars::delete(repo.clone(), cache, car_id).await?;
    Ok(())
}

// Example of end-to-end test with real database and repository
// 1. run `docker-compose -f compose-tests.yaml up -d` to start up the test db server
// 2. remove #[ignore] on the test method
#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::controllers::cars;
    use crate::models::car::{CarList, NewCar};
    use crate::repositories::car::CarRepository;
    use crate::repositories::{clear_database, create_car_repository, run_migrations};
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
        let real_repo = create_car_repository(&config).await;

        // given
        let car = NewCar {
            name: "Tesla".to_string(),
            color: Some("Red".to_string()),
            year: Some(2020),
        };
        real_repo.create(&car).await.unwrap();

        // Create an Axum router with the mock repository as an extension
        let app = Router::new()
            .route("/cars", get(cars::list))
            .layer(Extension(Arc::new(real_repo)));

        // Build a request to simulate a GET /cars
        let request = Request::builder()
            .uri("/cars?name=Tesla")
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
        let cars: CarList =
            serde_json::from_slice(&response_body).expect("Failed to deserialize response");
        assert_eq!(cars.data[0].name, "Tesla");
        assert_eq!(cars.data[0].color, Some("Red".to_string()));
        assert_eq!(cars.data[0].year, Some(2020));
    }
}

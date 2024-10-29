use crate::controllers::{utils, parts, cars};
use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

pub const CARS_TAG: &str = "Cars";
pub const PARTS_TAG: &str = "Parts";
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = CARS_TAG, description = "Cars management API"),
        (name = PARTS_TAG, description = "Parts management API")
    )
)]
struct ApiDoc;
pub fn router() -> Router {
    let app = OpenApiRouter::new()
        .routes(routes!(utils::healthcheck))
        .nest("/cars", car_routes())
        .nest("/parts", part_routes());

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", app)
        .split_for_parts();
    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // Alternative to above
        // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api));

    Router::new().nest("/", router)
}

fn car_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(cars::list))
        .routes(routes!(cars::search))
        .routes(routes!(cars::create))
        .routes(routes!(cars::view))
        .routes(routes!(cars::update))
        .routes(routes!(cars::delete))
}

fn part_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(parts::index))
        .routes(routes!(parts::search))
        .routes(routes!(parts::create))
        .routes(routes!(parts::view))
        .routes(routes!(parts::update))
        .routes(routes!(parts::delete))
}

use crate::controllers::{auth, cars, parts, users, utils};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::Http;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::SecurityScheme;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

pub const AUTH_TAG: &str = "Auth";
pub const USERS_TAG: &str = "Users";
pub const CARS_TAG: &str = "Cars";
pub const PARTS_TAG: &str = "Parts";

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}
#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = AUTH_TAG, description = "Auth management API"),
        (name = USERS_TAG, description = "Users management API"),
        (name = CARS_TAG, description = "Cars management API"),
        (name = PARTS_TAG, description = "Parts management API")
    )
)]
struct ApiDoc;
pub fn router() -> Router {
    let app = OpenApiRouter::new()
        .routes(routes!(utils::healthcheck))
        .routes(routes!(utils::save_request_body))
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
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
        .merge(Scalar::with_url("/scalar", api))
        // host SPA assets
        .fallback_service(
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html")),
        );

    Router::new().merge(router)
}

fn user_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(users::list))
        .routes(routes!(users::search))
        .routes(routes!(users::create))
        .routes(routes!(users::view))
        .routes(routes!(users::update))
        .routes(routes!(users::delete))
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

fn auth_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(auth::authorize))
        .routes(routes!(auth::test))
}

use crate::cache::create_cache;
use crate::config::Config;
use crate::repositories::{
    create_car_repository, create_part_repository, create_user_repository, run_migrations,
};
use crate::router::router;
use axum::extract::{MatchedPath, Request};
use axum::{Extension, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info_span;

pub async fn create_app(config: &Config) -> Router {
    let _ = run_migrations(config).await;
    let user_repository = Arc::new(create_user_repository(config).await);
    let car_repository = Arc::new(create_car_repository(config).await);
    let part_repository = Arc::new(create_part_repository(config).await);
    let cache = Arc::new(create_cache(config).await);
    router()
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();

                    // axum automatically adds this extension.
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    info_span!("request: ", %method, %uri, matched_path)
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        )
        .layer(Extension(user_repository))
        .layer(Extension(car_repository))
        .layer(Extension(part_repository))
        .layer(Extension(cache))
}

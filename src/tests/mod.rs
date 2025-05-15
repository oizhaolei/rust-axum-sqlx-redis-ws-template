pub mod fixture;

use axum::{Router, body::Body, http::Request, response::Response};
use tower::util::ServiceExt;

pub async fn request(app: Router, url: &'static str, body: Body) -> Response {
    app.oneshot(Request::builder().uri(url).body(body).unwrap())
        .await
        .unwrap()
}

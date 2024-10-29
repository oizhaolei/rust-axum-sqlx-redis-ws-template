pub mod parts;
pub mod cars;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::router;
    use crate::tests::request;
    use axum::{body::Body, http::StatusCode};

    #[tokio::test]
    async fn index() {
        let app = router::router();
        let response = request(app, "/api/healthcheck", Body::empty()).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
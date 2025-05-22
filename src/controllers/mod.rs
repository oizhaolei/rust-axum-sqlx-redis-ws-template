use serde::{Deserialize, Deserializer, Serialize};

pub mod auth;
pub mod cars;
pub mod parts;
pub mod users;
pub mod utils;

fn default_per_page() -> Option<usize> {
    Some(1000)
}

fn default_page() -> Option<usize> {
    Some(1)
}

fn default_field() -> Option<String> {
    Some("id".to_string())
}

fn default_order() -> Option<String> {
    Some("ASC".to_string())
}

fn default_ids() -> Vec<i32> {
    [].to_vec()
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    // page number: starts from `1`
    #[serde(default = "default_page")]
    pub page: Option<usize>,
    #[serde(default = "default_per_page")]
    pub per_page: Option<usize>,
    #[serde(default = "default_field")]
    pub field: Option<String>,
    #[serde(default = "default_order")]
    pub order: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Default, Clone, PartialEq)]
pub struct CommonQuery {
    #[serde(deserialize_with = "deserialize_string_to_array")]
    #[serde(default = "default_ids")]
    pub ids: Vec<i32>,
}

fn deserialize_string_to_array<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let array: Vec<i32> = s
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    Ok(array)
}

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

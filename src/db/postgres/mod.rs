use crate::config::Config;
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::sync::Arc;

pub type Db = Arc<Pool<Postgres>>;

pub async fn db_connect(config: &Config) -> Pool<Postgres> {
    dotenv().ok();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await
        .expect("Error connecting to database")
}

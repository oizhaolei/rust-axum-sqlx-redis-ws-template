use crate::db::postgres;
use crate::repositories::{
    part::{PartRepositoryImpl},
    car::{CarRepositoryImpl},
};
use axum::extract::Extension;
use std::sync::Arc;
use crate::config::Config;

pub mod part;
pub mod car;

pub type CarRepoExt = Extension<Arc<CarRepositoryImpl>>;
pub type PartRepoExt = Extension<Arc<PartRepositoryImpl>>;

pub async fn run_migrations(config: &Config) {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    if let Err(e) = sqlx::migrate!().run(&*db_pool).await {
        panic!("Failed to run database migrations: {:?}", e);
    }
}
pub async fn create_car_repository(config: &Config) -> CarRepositoryImpl {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    CarRepositoryImpl::new(db_pool.clone())
}

pub async fn create_part_repository(config: &Config) -> PartRepositoryImpl {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    PartRepositoryImpl::new(db_pool.clone())
}

#[cfg(test)]
pub async fn clear_database(config: &Config) {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    sqlx::query("TRUNCATE TABLE parts, cars CASCADE")
        .execute(&*db_pool)
        .await
        .expect("Failed to clear database tables");
}
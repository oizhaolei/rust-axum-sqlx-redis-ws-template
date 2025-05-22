use crate::controllers::{CommonQuery, Pagination};
use crate::db::postgres::Db;
use crate::models::part::{NewPart, Part, PartList, PartQuery};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

pub struct PartRepositoryImpl {
    pool: Db,
}
impl PartRepositoryImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[automock]
#[async_trait]
pub trait PartRepository {
    async fn find_all(
        &self,
        conditions: &PartQuery,
        query: &CommonQuery,
        pagination: &Pagination,
    ) -> Result<PartList>;
    async fn create(&self, part_data: &NewPart) -> Result<Part>;
    async fn update(&self, part_data: &Part) -> Result<Part>;
    async fn delete(&self, part_id: i32) -> Result<u64>;
    async fn find_by_id(&self, part_id: i32) -> Result<Part>;
}

#[async_trait]
impl PartRepository for PartRepositoryImpl {
    async fn find_all(&self,
        conditions: &PartQuery,
        query: &CommonQuery,
        pagination: &Pagination,
) -> Result<PartList> {
        let limit = pagination.per_page.unwrap_or(100);
        let offset = (pagination.page.unwrap_or(1) - 1) * limit;

        let data = if let Some(name) = &conditions.name {
            sqlx::query_as!(
                Part,
                "SELECT * FROM parts WHERE NAME LIKE $1 LIMIT $2 OFFSET $3",
                format!("%{}%", name),
                limit as i32,
                offset as i32,
            )
            .fetch_all(&*self.pool)
            .await?
        } else if !query.ids.is_empty() {
            sqlx::query_as!(
                Part,
                "SELECT * FROM parts WHERE id IN (SELECT unnest($1::integer[])) LIMIT $2 OFFSET $3",
                &query.ids,
                limit as i32,
                offset as i32,
            )
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query_as!(
                Part,
                "SELECT * FROM parts LIMIT $1 OFFSET $2",
                limit as i32,
                offset as i32
            )
            .fetch_all(&*self.pool)
            .await?
        };
        let total = if let Some(name) = &conditions.name {
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM parts WHERE NAME LIKE $1",
                format!("%{}%", name),
            )
            .fetch_one(&*self.pool)
            .await?
            .unwrap()
        } else if !query.ids.is_empty() {
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM parts WHERE id IN (SELECT unnest($1::integer[]))",
                &query.ids,
            )
            .fetch_one(&*self.pool)
            .await?
            .unwrap()
        } else {
            sqlx::query_scalar!("SELECT COUNT(*) FROM parts")
                .fetch_one(&*self.pool)
                .await?
                .unwrap()
        };

        Ok(PartList { data, total })
    }

    async fn create(&self, part_data: &NewPart) -> Result<Part> {
        let created_part = sqlx::query_as!(
            Part,
            r#"
            INSERT INTO parts (name, car_id)
            VALUES ($1, $2)
            RETURNING id, name, car_id
            "#,
            &part_data.name,
            part_data.car_id,
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(created_part)
    }

    async fn update(&self, part_data: &Part) -> Result<Part> {
        let updated_part = sqlx::query_as!(
            Part,
            r#"
            UPDATE parts
            SET name = $2, car_id = $3
            WHERE id = $1
            RETURNING id, name, car_id
            "#,
            part_data.id,
            &part_data.name,
            part_data.car_id,
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(updated_part)
    }

    async fn delete(&self, part_id: i32) -> Result<u64> {
        let query = sqlx::query("DELETE FROM parts WHERE id = $1")
            .bind(part_id)
            .execute(&*self.pool)
            .await?;
        Ok(query.rows_affected())
    }

    async fn find_by_id(&self, part_id: i32) -> Result<Part> {
        let row = sqlx::query_as!(Part, "SELECT * FROM parts WHERE id = $1", part_id,)
            .fetch_one(&*self.pool)
            .await?;
        Ok(row)
    }
}

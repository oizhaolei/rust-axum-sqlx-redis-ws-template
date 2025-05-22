use crate::controllers::{CommonQuery, Pagination};
use crate::db::postgres::Db;
use crate::models::car::{Car, CarList, CarQuery, NewCar};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

pub struct CarRepositoryImpl {
    pool: Db,
}
impl CarRepositoryImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[automock]
#[async_trait]
pub trait CarRepository {
    async fn find_all(
        &self,
        conditions: &CarQuery,
        query: &CommonQuery,
        pagination: &Pagination,
    ) -> Result<CarList>;
    async fn create(&self, car_data: &NewCar) -> Result<Car>;
    async fn update(&self, car_data: &Car) -> Result<Car>;
    async fn delete(&self, car_id: i32) -> Result<u64>;
    async fn find_by_id(&self, car_id: i32) -> Result<Car>;
}

#[async_trait]
impl CarRepository for CarRepositoryImpl {
    async fn find_all(
        &self,
        conditions: &CarQuery,
        query: &CommonQuery,
        pagination: &Pagination,
    ) -> Result<CarList> {
        let limit = pagination.per_page.unwrap_or(100);
        let offset = (pagination.page.unwrap_or(1) - 1) * limit;

        let data = if let Some(name) = &conditions.name {
            sqlx::query_as!(
                Car,
                "SELECT * FROM cars WHERE NAME LIKE $1 LIMIT $2 OFFSET $3",
                format!("%{}%", name),
                limit as i32,
                offset as i32,
            )
            .fetch_all(&*self.pool)
            .await?
        } else if !query.ids.is_empty() {
            sqlx::query_as!(
                Car,
                "SELECT * FROM cars WHERE id IN (SELECT unnest($1::integer[])) LIMIT $2 OFFSET $3",
                &query.ids,
                limit as i32,
                offset as i32,
            )
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query_as!(
                Car,
                "SELECT * FROM cars LIMIT $1 OFFSET $2",
                limit as i32,
                offset as i32
            )
            .fetch_all(&*self.pool)
            .await?
        };
        let total = if let Some(name) = &conditions.name {
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM cars WHERE NAME LIKE $1",
                format!("%{}%", name),
            )
            .fetch_one(&*self.pool)
            .await?
            .unwrap()
        } else if !query.ids.is_empty() {
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM cars WHERE id IN (SELECT unnest($1::integer[]))",
                &query.ids,
            )
            .fetch_one(&*self.pool)
            .await?
            .unwrap()
        } else {
            sqlx::query_scalar!("SELECT COUNT(*) FROM cars")
                .fetch_one(&*self.pool)
                .await?
                .unwrap()
        };

        Ok(CarList { data, total })
    }

    async fn create(&self, car_data: &NewCar) -> Result<Car> {
        let created_car = sqlx::query_as::<_, Car>(
            r#"
            INSERT INTO cars (name, color, year)
            VALUES ($1, $2, $3)
            RETURNING id, name, color, year
            "#,
        )
        .bind(&car_data.name)
        .bind(&car_data.color)
        .bind(car_data.year)
        .fetch_one(&*self.pool)
        .await?;
        Ok(created_car)
    }

    async fn update(&self, car_data: &Car) -> Result<Car> {
        let updated_car = sqlx::query_as::<_, Car>(
            r#"
            UPDATE cars
            SET name = $2, color = $3, year = $4
            WHERE id = $1
            RETURNING id, name, color, year
            "#,
        )
        .bind(car_data.id)
        .bind(&car_data.name)
        .bind(&car_data.color)
        .bind(car_data.year)
        .fetch_one(&*self.pool)
        .await?;
        Ok(updated_car)
    }

    async fn delete(&self, car_id: i32) -> Result<u64> {
        let query = sqlx::query("DELETE FROM cars WHERE id = $1")
            .bind(car_id)
            .execute(&*self.pool)
            .await?;
        Ok(query.rows_affected())
    }

    async fn find_by_id(&self, car_id: i32) -> Result<Car> {
        let row = sqlx::query_as!(Car, "SELECT * FROM cars WHERE id = $1", car_id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate;
    #[tokio::test]
    async fn test_find_all_cars() {
        let mut mock_repo = MockCarRepository::new();
        let conditions = CarQuery {
            name: Some("Tesla".to_string()),
        };
        let query = CommonQuery { ids: [].to_vec() };
        let pagination = Pagination {
            page: None,
            per_page: None,
            field: None,
            order: None,
        };
        let expected_cars = CarList {
            data: vec![
                Car {
                    id: 1,
                    name: "Tesla Model S".to_string(),
                    color: None,
                    year: None,
                },
                Car {
                    id: 2,
                    name: "Tesla Model 3".to_string(),
                    color: None,
                    year: None,
                },
            ],
            total: 99,
        };

        mock_repo
            .expect_find_all()
            .with(
                predicate::eq(conditions.clone()),
                predicate::eq(query.clone()),
                predicate::eq(pagination.clone()),
            )
            .times(1)
            .returning(move |_, _, _| Ok(expected_cars.clone()));

        let result = mock_repo.find_all(&conditions, &query, &pagination).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().data.len(), 2);
    }
}

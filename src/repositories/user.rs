use crate::db::postgres::Db;
use crate::models::user::{User, UserAuth, UserList, UserQuery};
use crate::password;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

pub struct UserRepositoryImpl {
    pool: Db,
}
impl UserRepositoryImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[automock]
#[async_trait]
pub trait UserRepository {
    async fn find_all(&self, conditions: &UserQuery) -> Result<UserList>;
    async fn create(&self, user_data: &UserAuth) -> Result<User>;
    async fn update(&self, user_data: &UserAuth) -> Result<User>;
    async fn delete(&self, username: &str) -> Result<u64>;
    async fn find_by_username(&self, username: &str) -> Result<User>;
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_all(&self, conditions: &UserQuery) -> Result<UserList> {
        let result = if let Some(username) = &conditions.username {
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE USERNAME LIKE $1",
                format!("%{}%", username)
            )
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query_as!(User, "SELECT * FROM users")
                .fetch_all(&*self.pool)
                .await?
        };

        Ok(result)
    }

    async fn create(&self, user_data: &UserAuth) -> Result<User> {
        let password_hash = password::hash(user_data.password.to_string()).await?;

        let created_user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, password_hash )
            VALUES ($1, $2)
            RETURNING username, password_hash
            "#,
        )
        .bind(&user_data.username)
        .bind(password_hash)
        .fetch_one(&*self.pool)
        .await?;
        Ok(created_user)
    }

    async fn update(&self, user_data: &UserAuth) -> Result<User> {
        let password_hash = password::hash(user_data.password.to_string()).await?;
        let updated_user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET password_hash = $1
            WHERE username = $2
            RETURNING password_hash, username
            "#,
        )
        .bind(&user_data.username)
        .bind(password_hash)
        .fetch_one(&*self.pool)
        .await?;
        Ok(updated_user)
    }

    async fn delete(&self, username: &str) -> Result<u64> {
        let query = sqlx::query("DELETE FROM users WHERE username = $1")
            .bind(username)
            .execute(&*self.pool)
            .await?;
        Ok(query.rows_affected())
    }

    async fn find_by_username(&self, username: &str) -> Result<User> {
        let row = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
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
    async fn test_find_all_users() {
        let mut mock_repo = MockUserRepository::new();
        let conditions = UserQuery {
            username: Some("Tesla".to_string()),
        };
        let expected_users = vec![
            User {
                username: "Tesla Model S".to_string(),
                password_hash: "None".to_string(),
            },
            User {
                username: "Tesla Model 3".to_string(),
                password_hash: "None".to_string(),
            },
        ];

        mock_repo
            .expect_find_all()
            .with(predicate::eq(conditions.clone()))
            .times(1)
            .returning(move |_| Ok(expected_users.clone()));

        let result = mock_repo.find_all(&conditions).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
}

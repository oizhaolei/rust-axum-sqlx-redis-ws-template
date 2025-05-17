use crate::models::user::{User, UserAuth, UserList, UserQuery};
use crate::repositories::user::UserRepository;
use anyhow::{Result, bail};
use std::sync::Arc;
use tracing::info;

pub async fn search<R: UserRepository>(repo: Arc<R>, conditions: &UserQuery) -> Result<UserList> {
    let users = repo.find_all(conditions).await?;
    Ok(users)
}

pub async fn view<R: UserRepository>(repo: Arc<R>, username: &str) -> Result<User> {
    info!("Fetching user {} from db...", username);
    // query the database
    let user = repo.find_by_username(username).await?;

    Ok(user)
}

pub async fn create<R: UserRepository>(repo: Arc<R>, new_user: &UserAuth) -> Result<User> {
    let user = repo.create(new_user).await?;
    Ok(user)
}

pub async fn update<R: UserRepository>(repo: Arc<R>, user: &UserAuth) -> Result<User> {
    let user = repo.update(user).await?;
    Ok(user)
}

pub async fn login<R: UserRepository>(repo: Arc<R>, user: &UserAuth) -> Result<User> {
    // Check if the user sent the credentials
    if user.username.is_empty() || user.password.is_empty() {
        anyhow::bail!("MissingCredentials");
    }

    let db_user = repo.find_by_username(&user.username).await?;
    //verrify
    let verified =
        crate::password::verify(user.password.clone(), db_user.password_hash.to_string()).await?;
    if !verified {
        bail!("invalid login {} / {}", &user.username, &user.password);
    }

    Ok(db_user)
}

pub async fn delete<R: UserRepository>(repo: Arc<R>, username: &str) -> Result<u64> {
    let affected_rows = repo.delete(username).await?;
    if affected_rows == 0 {
        bail!("No rows affected, user with ID {} not found", username);
    } else if affected_rows > 1 {
        bail!("Unexpected number of rows affected: {}", affected_rows);
    }
    Ok(affected_rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user::MockUserRepository;
    use crate::tests::fixture::user::users_fixture;

    #[tokio::test]
    async fn test_search() {
        let mut mock_repo_impl = MockUserRepository::new();
        mock_repo_impl
            .expect_find_all()
            .returning(|_| Ok(users_fixture(5)));
        let conditions = UserQuery { username: None };
        let users = search(Arc::new(mock_repo_impl), &conditions).await.unwrap();
        assert_eq!(users.len(), 5);
    }
}

use crate::cache::CacheImpl;
use crate::controllers::{CommonQuery, Pagination};
use crate::models::part::{NewPart, Part, PartList, PartQuery};
use crate::repositories::part::PartRepository;
use anyhow::{Result, bail};
use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8::PooledConnection;
use redis::AsyncCommands;
use std::sync::Arc;
use tracing::info;
use validator::Validate;

const PART_CACHE_TTL: u64 = 60;

pub async fn find_all<R: PartRepository>(
    repo: Arc<R>,
    conditions: &PartQuery,
    query: &CommonQuery,
    pagination: &Pagination,
) -> Result<PartList> {
    let parts = repo.find_all(conditions, query, pagination).await?;
    Ok(parts)
}

pub async fn view<R: PartRepository>(
    repo: Arc<R>,
    cache: Arc<CacheImpl>,
    part_id: i32,
) -> Result<Part> {
    // Construct the cache key
    let cache_key = format!("part:{}", part_id);
    let mut redis_conn: PooledConnection<RedisConnectionManager> = cache.redis_pool.get().await?;
    // Attempt to retrieve the part data from cache
    let maybe_cached: Option<String> = redis_conn.get::<String, _>(cache_key.clone()).await?;

    if let Some(cached_json) = maybe_cached {
        // Deserialize the JSON string back to a Part object
        let cached_part: Part = serde_json::from_str(&cached_json)?;
        info!("Found cached part {}: {:?}", part_id, cached_part);
        return Ok(cached_part);
    }
    info!("Fetching part {} from db...", part_id);
    // If not in cache, query the database
    let part = repo.find_by_id(part_id).await?;

    // Serialize the part object to a JSON string
    let part_json = serde_json::to_string(&part)?;

    // Store the serialized Part in the cache
    redis_conn
        .set_ex::<_, _, ()>(&cache_key, part_json, PART_CACHE_TTL)
        .await?;

    Ok(part)
}

pub async fn create<R: PartRepository>(repo: Arc<R>, new_part: &NewPart) -> Result<Part> {
    new_part.validate()?;
    let part = repo.create(new_part).await?;
    Ok(part)
}

pub async fn update<R: PartRepository>(
    repo: Arc<R>,
    cache: Arc<CacheImpl>,
    part: &Part,
) -> Result<Part> {
    // Construct the cache key
    let cache_key = format!("part:{}", part.id);
    let mut redis_conn: PooledConnection<RedisConnectionManager> = cache.redis_pool.get().await?;
    // Attempt to retrieve the part data from cache
    let _: Option<String> = redis_conn.del::<String, _>(cache_key.clone()).await?;

    let part = repo.update(part).await?;
    Ok(part)
}

pub async fn delete<R: PartRepository>(
    repo: Arc<R>,
    cache: Arc<CacheImpl>,
    part_id: i32,
) -> Result<u64> {
    // Construct the cache key
    let cache_key = format!("part:{}", part_id);
    let mut redis_conn: PooledConnection<RedisConnectionManager> = cache.redis_pool.get().await?;
    // Attempt to retrieve the part data from cache
    let _: Option<String> = redis_conn.del::<String, _>(cache_key.clone()).await?;

    let affected_rows = repo.delete(part_id).await?;
    if affected_rows == 0 {
        bail!("No rows affected, part with ID {} not found", part_id);
    } else if affected_rows > 1 {
        bail!("Unexpected number of rows affected: {}", affected_rows);
    }
    Ok(affected_rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::part::MockPartRepository;
    use crate::tests::fixture::part::parts_fixture;

    #[tokio::test]
    async fn test_find_all() {
        let mut mock_repo_impl = MockPartRepository::new();
        mock_repo_impl
            .expect_find_all()
            .returning(|_, _, _| Ok(parts_fixture(5)));
        let conditions = PartQuery { name: None };
        let query = CommonQuery { ids: [].to_vec() };
        let pagination = Pagination {
            page: None,
            per_page: None,
            field: None,
            order: None,
        };
        let parts = find_all(Arc::new(mock_repo_impl), &conditions, &query, &pagination)
            .await
            .unwrap();
        assert_eq!(parts.data.len(), 5);
    }
}

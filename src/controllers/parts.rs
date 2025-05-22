use crate::cache::CacheExt;
use crate::error::{AppError, AppJson};
use crate::models::part::{NewPart, Part, PartList, PartQuery};
use crate::repositories::PartRepoExt;
use crate::router::PARTS_TAG;
use crate::services;
use axum::{
    Json,
    extract::{Extension, Path},
};
use axum_extra::extract::Query;

use super::auth::Claims;
use super::{CommonQuery, Pagination};

/// List Parts
///
/// Tries to all Parts from the database.
#[utoipa::path(
    get,
    path = "/list",
    params(
        ("name" = inline(Option<String>), Query, description="Part Name"),
        ("ids" = inline(Option<String>), Query, description="ids"),
        ("page" = inline(Option<usize>), Query, description="Page"),
        ("perPage" = inline(Option<usize>), Query, description="PerPage"),
        ("field" = inline(Option<String>), Query, description="Field"),
        ("order" = inline(Option<String>), Query, description="Order")
    ) ,
    responses((status = OK, body = PartList)),
    tag = PARTS_TAG
)]
pub async fn list(
    Query(conditions): Query<PartQuery>,
    Query(query): Query<CommonQuery>,
    Query(pagination): Query<Pagination>,
    Extension(repo): PartRepoExt,
) -> Result<AppJson<PartList>, AppError> {
    println!("list params: {:?}", pagination);
    println!("conditions: {:?}", conditions);
    println!("ids: {:?}", query);
    let parts = services::parts::find_all(repo.clone(), &conditions, &query, &pagination).await?;
    Ok(AppJson(parts))
}

/// Get single Part by id
///
/// Tries to get single part by id from the database
#[utoipa::path(
    get,
    path = "/{part_id}",
    params(("part_id" = i32, Path, description="Part Id")),
    responses((status = OK, body = Part)),
    tag = PARTS_TAG
)]
pub async fn view(
    _claims: Claims,
    Path(part_id): Path<i32>,
    Extension(repo): PartRepoExt,
    Extension(cache): CacheExt,
) -> Result<AppJson<Part>, AppError> {
    let part = services::parts::view(repo.clone(), cache.clone(), part_id).await?;
    Ok(AppJson(part))
}

/// Create new Part
///
/// Tries to create a new Part in the database.
#[utoipa::path(
        post,
        path = "/create",
        tag = PARTS_TAG,
        security(
            ("bearerAuth" = [])
        ),
        request_body(content=NewPart, content_type="application/json", description="New Part Information"),
        responses(
            (status = 201, description = "Part item created successfully", body = Part)
        )
)]
pub async fn create(
    _claims: Claims,
    Extension(repo): PartRepoExt,
    Json(new_part): Json<NewPart>,
) -> Result<AppJson<Part>, AppError> {
    let part = services::parts::create(repo.clone(), &new_part).await?;
    Ok(AppJson(part))
}

/// Update existing Part
///
/// Tries to update a Part in the database.
#[utoipa::path(
        post,
        path = "/update",
        tag = PARTS_TAG,
        security(
            ("bearerAuth" = [])
        ),
        request_body(content=Part, content_type="application/json", description="Part To Update"),
        responses(
            (status = 200, description = "Part item updated successfully", body = Part)
        )
)]
pub async fn update(
    _claims: Claims,
    Extension(repo): PartRepoExt,
    Extension(cache): CacheExt,
    Json(part): Json<Part>,
) -> Result<AppJson<Part>, AppError> {
    let part = services::parts::update(repo.clone(), cache, &part).await?;
    Ok(AppJson(part))
}

/// Delete existing Part
///
/// Tries to delete a Part from the database.
#[utoipa::path(
        delete,
        path = "/delete/{part_id}",
        params(("part_id" = i32, Path, description="Part Id")),
        tag = PARTS_TAG,
        security(
            ("bearerAuth" = [])
        ),
        responses(
            (status = 200, description = "Part item deleted successfully", body = String)
        )
)]
pub async fn delete(
    _claims: Claims,
    Path(part_id): Path<i32>,
    Extension(repo): PartRepoExt,
    Extension(cache): CacheExt,
) -> Result<(), AppError> {
    services::parts::delete(repo.clone(), cache, part_id).await?;
    Ok(())
}

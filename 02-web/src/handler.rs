use uuid::Uuid;
use axum::{extract::{Extension, Json, Path}, http::StatusCode};
use crate::{model::*, db::DbPool};

// 创建用户
pub async fn create_user(
    Extension(pool): Extension<DbPool>,
    Json(user_data): Json<CreateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    match UserStore::create(&pool, &user_data).await {
        Ok(user) => Ok(Json(user)),
        Err(UserError::EmailExists) => Err((StatusCode::CONFLICT, "邮箱已存在".to_string())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// 获取所有用户
pub async fn get_all_users(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match UserStore::find_all(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// 获取单个用户
pub async fn get_user(
    Extension(pool): Extension<DbPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, (StatusCode, String)> {
    match UserStore::find_by_id(&pool, user_id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "用户不存在".to_string())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// 更新用户
pub async fn update_user(
    Extension(pool): Extension<DbPool>,
    Path(user_id): Path<Uuid>,
    Json(update_data): Json<UpdateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    match UserStore::update(&pool, user_id, &update_data).await {
        Ok(user) => Ok(Json(user)),
        Err(UserError::NotFound) => Err((StatusCode::NOT_FOUND, "用户不存在".to_string())),
        Err(UserError::EmailExists) => Err((StatusCode::CONFLICT, "邮箱已存在".to_string())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// 删除用户
pub async fn delete_user(
    Extension(pool): Extension<DbPool>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    match UserStore::delete(&pool, user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(UserError::NotFound) => Err((StatusCode::NOT_FOUND, "用户不存在".to_string())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

// 用户错误类型
#[derive(Error, Debug)]
pub enum UserError {
    #[error("用户不存在")]
    NotFound,
    #[error("邮箱已存在")]
    EmailExists,
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
}

// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

// 用户存储实现
pub struct UserStore;

impl UserStore {
    // 创建新用户
    pub async fn create(pool: &PgPool, user_data: &CreateUserRequest) -> Result<User, UserError> {
        // 检查邮箱是否已存在
        if let Ok(Some(_)) = Self::find_by_email(pool, &user_data.email).await {
            return Err(UserError::EmailExists);
        }

        // 创建用户
        let user = sqlx::query_as::<_, User>(r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, name, email, password, created_at, updated_at
            "#)
            .bind(&user_data.name)
            .bind(&user_data.email)
            .bind(&user_data.password)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    // 获取所有用户
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, UserError> {
        let users = sqlx::query_as::<_, User>(r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#)
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    // 根据ID查找用户
    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as::<_, User>(r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users
            WHERE id = $1
            "#)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    // 根据邮箱查找用户
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as::<_, User>(r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users
            WHERE email = $1
            "#)
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    // 更新用户
    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        update_data: &UpdateUserRequest
    ) -> Result<User, UserError> {
        // 检查用户是否存在
        let existing_user = Self::find_by_id(pool, user_id).await?
            .ok_or(UserError::NotFound)?;

        // 构建更新查询
        let name = update_data.name.as_ref().unwrap_or(&existing_user.name);
        let email = update_data.email.as_ref().unwrap_or(&existing_user.email);
        let password = update_data.password.as_ref().unwrap_or(&existing_user.password);

        // 如果更新了邮箱，检查新邮箱是否已存在
        if email != &existing_user.email {
            if let Ok(Some(_)) = Self::find_by_email(pool, email).await {
                return Err(UserError::EmailExists);
            }
        }

        // 执行更新
        let updated_user = sqlx::query_as::<_, User>(r#"
            UPDATE users
            SET name = $1, email = $2, password = $3
            WHERE id = $4
            RETURNING id, name, email, password, created_at, updated_at
            "#)
            .bind(name)
            .bind(email)
            .bind(password)
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        Ok(updated_user)
    }

    // 删除用户
    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), UserError> {
        // 检查用户是否存在
        if Self::find_by_id(pool, user_id).await?.is_none() {
            return Err(UserError::NotFound);
        }

        // 执行删除
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
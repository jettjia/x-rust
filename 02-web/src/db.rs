use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

// 创建数据库连接池
pub async fn create_pool() -> anyhow::Result<DbPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

// 运行数据库迁移
pub async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    // 先删除表，确保使用更新后的结构（实际生产环境中应使用ALTER TABLE）
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(pool)
        .await?;

    // 创建users表
    sqlx::query(
        r#"
        CREATE TABLE users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) NOT NULL UNIQUE,
            password VARCHAR(100) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // 创建更新时间的函数
    sqlx::query(
        r#"
        CREATE OR REPLACE FUNCTION update_updated_at()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;
        "#
    )
    .execute(pool)
    .await?;

    // 创建更新时间的触发器
    sqlx::query(
        r#"
        DO $$ 
        BEGIN
            IF NOT EXISTS (
                SELECT 1 
                FROM information_schema.triggers 
                WHERE trigger_name = 'update_users_updated_at'
                AND event_object_table = 'users'
            ) THEN
                CREATE TRIGGER update_users_updated_at
                BEFORE UPDATE ON users
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at();
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
use axum::{
    routing::get,
    Router,
    Extension,
};
use crate::db::DbPool;
use crate::handler::{create_user, get_all_users, get_user, update_user, delete_user};

// 创建路由
pub fn create_router(pool: DbPool) -> Router {
    // 创建路由并添加数据库连接池作为扩展
    Router::new()
        // 用户CRUD路由
        .route("/users", get(get_all_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        // 添加数据库连接池作为扩展
        .layer(Extension(pool))
}

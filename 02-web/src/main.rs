mod db;
mod handler;
mod model;
mod router;

use axum::serve;
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();

    // 设置日志级别
    tracing_subscriber::fmt::init();

    // 创建数据库连接池
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    // 运行数据库迁移
    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    // 创建路由
    let app = router::create_router(pool).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    );

    // 绑定地址并启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");
    tracing::info!("Server running on http://{}", addr);

    serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// 优雅关闭服务器
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down server...");
}

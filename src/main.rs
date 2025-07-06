use std::sync::Arc;

use crate::{
    config::{get_db_url, load_config},
    routes::app_routes,
};
use axum::{Router, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};

mod config;
mod controllers;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

// 应用状态结构体
pub struct AppState {
    pub db_pool: PgPool, // 数据库连接池
}

#[tokio::main]
async fn main() {
    load_config();

    // 连接数据库
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(get_db_url().as_str())
        .await
        .expect("连接数据库失败");

    let shared_state = Arc::new(AppState { db_pool });

    // 启动服务器
    let app = app_routes().with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

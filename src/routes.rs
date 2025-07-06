use std::sync::Arc;

use axum::Router;

use crate::{AppState, controllers::user_controllers::user_router};

/// 创建并返回应用程序的主路由。
///
/// 该函数将用户相关的路由（通过 `user_router()` 提供）合并到主路由中，
/// 并返回一个配置好的 `Router` 实例，用于处理所有的 HTTP 请求。
pub fn app_routes() -> Router<Arc<AppState>> {
    Router::new().merge(user_router())
}

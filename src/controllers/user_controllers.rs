use std::sync::Arc;

use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::method_routing::get,
};

use crate::{AppState, models::user_model::User, services::user_service::get_user_service};

/// 处理获取用户信息的异步处理函数。
///
/// # 参数
/// - `pool`: 数据库连接池，用于执行数据库操作。
/// - `user_id`: 需要查询的用户ID。
///
/// # 返回
/// 返回一个包含用户信息的 `Json<User>` 对象。
/// 如果查询成功，返回对应用户信息；
/// 如果查询失败，返回一个默认的用户对象（id为0，name为"N/A"，email为空）。
async fn get_user_handler(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> Json<User> {
    match get_user_service(&state.db_pool, id).await {
        Ok(user) => Json(user),
        Err(_) => Json(User {
            id: 0,
            name: "N/A".to_string(),
            email: "".to_string(),
        }),
    }
}

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new().route("/users/{id}", get(get_user_handler))
}

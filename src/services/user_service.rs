/// 根据用户ID获取用户信息的服务函数。
///
/// # 参数
/// - `pool`: 数据库连接池的引用，用于执行数据库操作。
/// - `user_id`: 需要查询的用户ID。
///
/// # 返回值
/// 返回一个 `Result<User, String>`，
/// - 成功时返回 `User` 实体；
/// - 失败时返回错误信息字符串（如“用户不存在”）。
///
/// # 异步
/// 该函数为异步函数，需要在异步环境中调用。
use sqlx::postgres::PgPool;

use crate::{models::user_model::User, repositories::user_repository::get_user_by_id};

pub async fn get_user_service(pool: &PgPool, user_id: i32) -> Result<User, String> {
    match get_user_by_id(pool, user_id).await {
        Ok(user) => Ok(user),
        Err(_) => Err("用户不存在".to_string()),
    }
}

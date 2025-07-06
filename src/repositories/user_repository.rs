use crate::models::user_model::User;
use sqlx::postgres::PgPool;

/// 根据用户ID获取用户信息
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `user_id`: 用户ID
///
/// # 返回
/// 返回包含用户信息的Result
pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, sqlx::Error> {
    // 执行SQL查询，根据用户ID查找用户
    sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
}

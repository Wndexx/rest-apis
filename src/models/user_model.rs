use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// 用户结构体，表示系统中的一个用户。
///
/// # 字段
/// - `id`: 用户的唯一标识符
/// - `name`: 用户名
/// - `email`: 用户的电子邮件地址
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

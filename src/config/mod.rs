use dotenv::dotenv;
use std::env;

/// 加载 .env 文件中的环境变量
pub fn load_config() {
    dotenv().ok();
}

/// 获取环境变量中的 DATABASE_URL 变量
///
/// # 返回值
///
/// DATABASE_URL 变量的值
pub fn get_db_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL 为空")
}
